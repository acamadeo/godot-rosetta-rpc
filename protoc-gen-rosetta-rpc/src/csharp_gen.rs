//! C# code generator: renders Askama templates against pre-resolved
//! [`ServiceView`]s. No aggregator file is needed -- each generated file
//! declares its own `namespace` line.

use askama::Template;
use heck::ToShoutySnakeCase;
use prost_types::compiler::code_generator_response::File;

use crate::generator::{LanguageGenerator, glue_template, render_file};
use crate::ir::{Ir, ServiceIr};
use crate::naming;
use crate::options::Options;
use crate::view::{MethodView, ServiceView};

pub struct CSharpGenerator;

impl LanguageGenerator for CSharpGenerator {
    fn generate(&self, ir: &Ir, _options: &Options) -> Result<Vec<File>, String> {
        let views: Vec<ServiceView> = ir
            .services
            .iter()
            .map(|service| build_service_view(service, ir))
            .collect::<Result<_, _>>()?;

        let mut files = Vec::new();

        for view in &views {
            let dir = naming::glue_output_dir(&view.package);

            // Every service in scope gets an interface, descriptors, a
            // client, and an Adapter — any language may need to *call* any
            // service, and whether a language actually *implements* one is
            // decided at runtime by `ServiceImplementations` (see
            // Bootstrap.cs.jinja), not at generation time.
            files.push(render_file(
                &dir,
                &view.type_name,
                "cs",
                ServiceTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{}Descriptors", view.type_name),
                "cs",
                DescriptorsTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{}Client", view.type_name),
                "cs",
                ClientTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{}Adapter", view.type_name),
                "cs",
                AdapterTemplate::from(view),
            )?);
        }

        // Aggregate outputs (ServiceFactory, Bootstrap) live under the
        // "dominant" namespace: the first service's. See the same caveat in
        // rust_gen.rs.
        let dominant_namespace = views.first().map(|v| v.package.clone()).unwrap_or_default();
        let dominant_dir = naming::glue_output_dir(&dominant_namespace);

        let service_factory_tpl = ServiceFactoryTemplate {
            package: &dominant_namespace,
            services: &views,
        };
        files.push(render_file(
            &dominant_dir,
            "GeneratedServiceFactory",
            "cs",
            service_factory_tpl,
        )?);

        let bootstrap_tpl = BootstrapTemplate {
            package: &dominant_namespace,
            services: &views,
        };
        files.push(render_file(
            &dominant_dir,
            "Bootstrap",
            "cs",
            bootstrap_tpl,
        )?);

        // Must be its own file: Godot resolves a C# script's instantiable
        // type by matching the file name, not by scanning the file for
        // whichever Godot-derived type it contains, so CSharpRuntime cannot
        // share a file with Bootstrap/ServiceImplementations (see the
        // comment in CSharpRuntime.cs.jinja).
        let runtime_tpl = CSharpRuntimeTemplate {
            package: &dominant_namespace,
        };
        files.push(render_file(
            &dominant_dir,
            "CSharpRuntime",
            "cs",
            runtime_tpl,
        )?);

        Ok(files)
    }
}

fn build_service_view(service: &ServiceIr, ir: &Ir) -> Result<ServiceView, String> {
    let file_info = ir
        .resolver
        .resolve(&format!("{}.{}", service.package, service.name))
        .ok_or_else(|| {
            format!(
                "could not resolve service: {}.{}",
                service.package, service.name
            )
        })?;
    let namespace = naming::csharp_namespace(file_info);

    let methods = service
        .methods
        .iter()
        .map(|method| {
            Ok(MethodView {
                proto_name: method.name.clone(),
                // Proto method names in this repo are already PascalCase,
                // matching C# convention directly — no casing transform.
                method_name: method.name.clone(),
                const_name: method.name.to_shouty_snake_case(),
                input_type: resolve_csharp_type(ir, &method.input.full_name)?,
                output_type: resolve_csharp_type(ir, &method.output.full_name)?,
            })
        })
        .collect::<Result<_, String>>()?;

    Ok(ServiceView {
        proto_name: service.name.clone(),
        // Repurposed for C# only: the resolved namespace, not the raw
        // dotted proto package — used both as the emitted `namespace ...;`
        // line and, via `naming::glue_output_dir`, the output directory.
        package: namespace,
        type_name: service.name.clone(),
        interface_name: format!("I{}", service.name),
        factory_method_name: service.name.clone(),
        methods,
    })
}

fn resolve_csharp_type(ir: &Ir, full_name: &str) -> Result<String, String> {
    let file_info = ir
        .resolver
        .resolve(full_name)
        .ok_or_else(|| format!("could not resolve message type: {full_name}"))?;
    Ok(naming::csharp_message_path(file_info, full_name))
}

glue_template!(ServiceTemplate, "csharp/Service.cs.jinja");
glue_template!(ClientTemplate, "csharp/Client.cs.jinja");
glue_template!(AdapterTemplate, "csharp/Adapter.cs.jinja");
glue_template!(DescriptorsTemplate, "csharp/Descriptors.cs.jinja");

#[derive(Template)]
#[template(path = "csharp/ServiceFactory.cs.jinja", escape = "none")]
struct ServiceFactoryTemplate<'a> {
    package: &'a str,
    services: &'a [ServiceView],
}

#[derive(Template)]
#[template(path = "csharp/Bootstrap.cs.jinja", escape = "none")]
struct BootstrapTemplate<'a> {
    package: &'a str,
    services: &'a [ServiceView],
}

#[derive(Template)]
#[template(path = "csharp/CSharpRuntime.cs.jinja", escape = "none")]
struct CSharpRuntimeTemplate<'a> {
    package: &'a str,
}
