//! Kotlin code generator: renders Askama templates against pre-resolved
//! [`ServiceView`]s. Unlike Rust, Kotlin needs no aggregator file — each
//! generated file declares its own `package` line and the JVM compiler
//! doesn't require an explicit module tree.

use askama::Template;
use heck::{ToLowerCamelCase, ToShoutySnakeCase};
use prost_types::compiler::code_generator_response::File;

use crate::generator::{LanguageGenerator, glue_template, render_file};
use crate::ir::{Ir, ServiceIr};
use crate::naming;
use crate::options::Options;
use crate::view::{MethodView, ServiceView};

pub struct KotlinGenerator;

impl LanguageGenerator for KotlinGenerator {
    fn generate(&self, ir: &Ir, _options: &Options) -> Result<Vec<File>, String> {
        let views: Vec<ServiceView> = ir
            .services
            .iter()
            .map(|service| build_service_view(service, ir))
            .collect::<Result<_, _>>()?;

        let mut files = Vec::new();

        for (service, view) in ir.services.iter().zip(&views) {
            let dir = naming::glue_output_dir(&service.package);

            // Every service in scope gets an interface, descriptors, a
            // client, and an Adapter — any language may need to *call* any
            // service, and whether a language actually *implements* one is
            // decided at runtime by `ServiceImplementations` (see
            // Bootstrap.kt.jinja), not at generation time.
            files.push(render_file(
                &dir,
                &view.type_name,
                "kt",
                ServiceTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{}Descriptors", view.type_name),
                "kt",
                DescriptorsTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{}Client", view.type_name),
                "kt",
                ClientTemplate::from(view),
            )?);
            files.push(render_file(
                &dir,
                &format!("{}Adapter", view.type_name),
                "kt",
                AdapterTemplate::from(view),
            )?);
        }

        // Aggregate outputs (ServiceFactory, Bootstrap) live under the
        // "dominant" package: the first service's package. See the same
        // caveat in rust_gen.rs.
        let dominant_package = ir
            .services
            .first()
            .map(|s| s.package.clone())
            .unwrap_or_default();
        let dominant_dir = naming::glue_output_dir(&dominant_package);

        let service_factory_tpl = ServiceFactoryTemplate {
            package: &dominant_package,
            services: &views,
        };
        files.push(render_file(
            &dominant_dir,
            "GeneratedServiceFactory",
            "kt",
            service_factory_tpl,
        )?);

        let bootstrap_tpl = BootstrapTemplate {
            package: &dominant_package,
            services: &views,
        };
        files.push(render_file(
            &dominant_dir,
            "Bootstrap",
            "kt",
            bootstrap_tpl,
        )?);

        Ok(files)
    }
}

fn build_service_view(service: &ServiceIr, ir: &Ir) -> Result<ServiceView, String> {
    let methods = service
        .methods
        .iter()
        .map(|method| {
            Ok(MethodView {
                proto_name: method.name.clone(),
                method_name: method.name.to_lower_camel_case(),
                const_name: method.name.to_shouty_snake_case(),
                input_type: resolve_kotlin_type(ir, &method.input.full_name)?,
                output_type: resolve_kotlin_type(ir, &method.output.full_name)?,
            })
        })
        .collect::<Result<_, String>>()?;

    Ok(ServiceView {
        proto_name: service.name.clone(),
        package: service.package.clone(),
        type_name: service.name.clone(),
        interface_name: service.name.clone(),
        factory_method_name: service.name.to_lower_camel_case(),
        methods,
    })
}

fn resolve_kotlin_type(ir: &Ir, full_name: &str) -> Result<String, String> {
    let file_info = ir
        .resolver
        .resolve(full_name)
        .ok_or_else(|| format!("could not resolve message type: {full_name}"))?;
    Ok(naming::kotlin_message_path(file_info, full_name))
}

glue_template!(ServiceTemplate, "kotlin/Service.kt.jinja");
glue_template!(DescriptorsTemplate, "kotlin/Descriptors.kt.jinja");
glue_template!(ClientTemplate, "kotlin/Client.kt.jinja");
glue_template!(AdapterTemplate, "kotlin/Adapter.kt.jinja");

#[derive(Template)]
#[template(path = "kotlin/ServiceFactory.kt.jinja", escape = "none")]
struct ServiceFactoryTemplate<'a> {
    package: &'a str,
    services: &'a [ServiceView],
}

#[derive(Template)]
#[template(path = "kotlin/Bootstrap.kt.jinja", escape = "none")]
struct BootstrapTemplate<'a> {
    package: &'a str,
    services: &'a [ServiceView],
}
