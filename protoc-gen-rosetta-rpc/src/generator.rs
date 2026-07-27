/// Dispatches IR to the right language's generator.
use askama::Template;
use prost_types::compiler::code_generator_response::File;

use crate::ir::Ir;
use crate::options::{Lang, Options};

/// Adding a new language means adding a `Lang` variant (see `options.rs`),
/// a new `xxx_gen.rs` implementing [`LanguageGenerator`], and one match arm
/// here — nothing in `ir.rs`, `naming.rs`, or `main.rs` needs to change.
pub trait LanguageGenerator {
    fn generate(&self, ir: &Ir, options: &Options) -> Result<Vec<File>, String>;
}

pub fn generate(ir: &Ir, options: &Options) -> Result<Vec<File>, String> {
    let generator: Box<dyn LanguageGenerator> = match options.lang {
        Lang::Rust => Box::new(crate::rust_gen::RustGenerator),
        Lang::Kotlin => Box::new(crate::kotlin_gen::KotlinGenerator),
    };
    generator.generate(ir, options)
}

/// Renders `template` and wraps it as a `File` at `dir/stem.extension`.
/// Called by every per-language generator, so only the file extension needs to vary.
pub fn render_file<T: Template>(
    dir: &str,
    stem: &str,
    extension: &str,
    template: T,
) -> Result<File, String> {
    let content = template
        .render()
        .map_err(|e| format!("failed to render {stem}: {e}"))?;
    let name = if dir.is_empty() {
        format!("{stem}.{extension}")
    } else {
        format!("{dir}/{stem}.{extension}")
    };
    Ok(File {
        name: Some(name),
        insertion_point: None,
        content: Some(content),
        generated_code_info: None,
    })
}

/// Declares a `Service`/`Descriptors`/`Client`/`Adapter`-shaped template
/// struct plus its `From<&ServiceView>` impl. All four artifact kinds share
/// this exact shape in every language — only the `.jinja` template path
/// differs (Askama needs that at compile time, so the structs themselves
/// can't be unified across languages).
macro_rules! glue_template {
    ($name:ident, $path:literal) => {
        #[derive(askama::Template)]
        #[template(path = $path, escape = "none")]
        struct $name<'a> {
            package: &'a str,
            proto_name: &'a str,
            type_name: &'a str,
            methods: &'a [$crate::view::MethodView],
        }

        impl<'a> From<&'a $crate::view::ServiceView> for $name<'a> {
            fn from(v: &'a $crate::view::ServiceView) -> Self {
                Self {
                    package: &v.package,
                    proto_name: &v.proto_name,
                    type_name: &v.type_name,
                    methods: &v.methods,
                }
            }
        }
    };
}
pub(crate) use glue_template;
