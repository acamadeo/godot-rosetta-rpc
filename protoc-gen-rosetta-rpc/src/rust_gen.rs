//! Rust code generator: renders Askama templates against pre-resolved
//! [`ServiceView`]s, and hand-builds the one aggregator file
//! (`mod_tree.rs`) whose job is inherently a recursive tree — not a good fit
//! for a flat template loop.

use std::collections::BTreeMap;

use askama::Template;
use heck::{ToShoutySnakeCase, ToSnakeCase};
use prost_types::compiler::code_generator_response::File;

use crate::generator::{LanguageGenerator, glue_template, render_file};
use crate::ir::{Ir, ServiceIr};
use crate::naming;
use crate::options::Options;
use crate::view::{MethodView, ServiceView};

pub struct RustGenerator;

impl LanguageGenerator for RustGenerator {
    fn generate(&self, ir: &Ir, options: &Options) -> Result<Vec<File>, String> {
        let message_crate = options
            .message_crate
            .as_deref()
            .ok_or_else(|| "lang=rust requires the message_crate parameter".to_string())?;

        let views: Vec<ServiceView> = ir
            .services
            .iter()
            .map(|service| build_service_view(service, message_crate))
            .collect();

        let mut files = Vec::new();
        let mut tree = PackageNode::default();

        for (service, view) in ir.services.iter().zip(&views) {
            let dir = naming::glue_output_dir(&service.package);
            let stem = &view.factory_method_name;

            // Every service in scope gets an interface, descriptors, a
            // client, and an Adapter — any language may need to *call* any
            // service, and whether a language actually *implements* one is
            // decided at runtime by `ServiceImplementations` (see
            // bootstrap.rs.jinja), not at generation time.
            files.push(render_file(&dir, stem, "rs", ServiceTemplate::from(view))?);
            files.push(render_file(
                &dir,
                &format!("{stem}_descriptors"),
                "rs",
                DescriptorsTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{stem}_client"),
                "rs",
                ClientTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{stem}_adapter"),
                "rs",
                AdapterTemplate::from(view),
            )?);

            tree.insert(
                &service.package,
                vec![
                    stem.clone(),
                    format!("{stem}_descriptors"),
                    format!("{stem}_client"),
                    format!("{stem}_adapter"),
                ],
            );
        }

        // Aggregate outputs (ServiceFactory, Bootstrap) are placed under the
        // "dominant" package — the first service's package. A single
        // invocation spanning multiple distinct packages is not yet
        // supported; see README's "Publishing status" / known limitations.
        let dominant_package = ir
            .services
            .first()
            .map(|s| s.package.clone())
            .unwrap_or_default();
        let dominant_dir = naming::glue_output_dir(&dominant_package);

        let service_factory_tpl = ServiceFactoryTemplate { services: &views };
        files.push(render_file(
            &dominant_dir,
            "service_factory",
            "rs",
            service_factory_tpl,
        )?);

        let bootstrap_tpl = BootstrapTemplate { services: &views };
        files.push(render_file(
            &dominant_dir,
            "bootstrap",
            "rs",
            bootstrap_tpl,
        )?);

        tree.insert(
            &dominant_package,
            vec!["service_factory".to_string(), "bootstrap".to_string()],
        );

        files.push(File {
            name: Some("mod_tree.rs".to_string()),
            insertion_point: None,
            content: Some(tree.render()),
            generated_code_info: None,
        });

        Ok(files)
    }
}

fn build_service_view(service: &ServiceIr, message_crate: &str) -> ServiceView {
    let methods = service
        .methods
        .iter()
        .map(|method| MethodView {
            proto_name: method.name.clone(),
            method_name: method.name.to_snake_case(),
            const_name: method.name.to_shouty_snake_case(),
            input_type: naming::rust_message_path(message_crate, &method.input.full_name),
            output_type: naming::rust_message_path(message_crate, &method.output.full_name),
        })
        .collect();

    ServiceView {
        proto_name: service.name.clone(),
        package: service.package.clone(),
        type_name: service.name.clone(),
        factory_method_name: service.name.to_snake_case(),
        methods,
    }
}

glue_template!(ServiceTemplate, "rust/service.rs.jinja");
glue_template!(DescriptorsTemplate, "rust/descriptors.rs.jinja");
glue_template!(ClientTemplate, "rust/client.rs.jinja");
glue_template!(AdapterTemplate, "rust/adapter.rs.jinja");

#[derive(Template)]
#[template(path = "rust/service_factory.rs.jinja", escape = "none")]
struct ServiceFactoryTemplate<'a> {
    services: &'a [ServiceView],
}

#[derive(Template)]
#[template(path = "rust/bootstrap.rs.jinja", escape = "none")]
struct BootstrapTemplate<'a> {
    services: &'a [ServiceView],
}

/// A tree of proto package segments mirroring the source `.proto` package,
/// whose leaves are `include!`-able file stems (no extension). Rendered
/// once into a single `mod_tree.rs`, matching how `rust/protobuf-gen/src/lib.rs`
/// hand-stitches per-package prost output in the parent repo, generalized so
/// it isn't tied to any specific crate name.
#[derive(Default)]
struct PackageNode {
    children: BTreeMap<String, PackageNode>,
    files: Vec<String>,
}

impl PackageNode {
    fn insert(&mut self, package: &str, files: Vec<String>) {
        if package.is_empty() {
            self.files.extend(files);
            return;
        }
        let mut node: &mut PackageNode = self;
        for segment in package.split('.') {
            node = node.children.entry(segment.to_string()).or_default();
        }
        node.files.extend(files);
    }

    fn render(&self) -> String {
        let mut out = String::new();
        self.render_into("", &mut out, 0);
        out
    }

    fn render_into(&self, path_prefix: &str, out: &mut String, indent: usize) {
        let pad = "    ".repeat(indent);
        for stem in &self.files {
            out.push_str(&format!("{pad}include!(\"{path_prefix}{stem}.rs\");\n"));
        }
        for (name, child) in &self.children {
            out.push_str(&format!("{pad}pub mod {name} {{\n"));
            let child_prefix = format!("{path_prefix}{name}/");
            child.render_into(&child_prefix, out, indent + 1);
            out.push_str(&format!("{pad}}}\n"));
        }
    }
}
