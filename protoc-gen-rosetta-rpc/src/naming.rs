//! Resolves proto full names to per-language qualified paths, for two
//! distinct purposes:
//!
//! - *Message type references*: where does the already-generated prost/
//!   protobuf-kotlin message type for `.pkg.Foo` actually live?
//! - *Generated glue namespace*: where should this plugin's own generated
//!   service interfaces/descriptors/clients/adapters live, so they mirror the
//!   source `.proto` package instead of inventing their own namespace?

use heck::ToUpperCamelCase;

use crate::ir::FileInfo;

/// Resolves a message's fully-qualified proto name to the Rust path of its
/// prost-generated type, e.g. `rust_message_path("protobuf_gen",
/// "rosetta.example.CurrentTimeRequest")` ->
/// `protobuf_gen::rosetta::example::CurrentTimeRequest`.
///
/// `message_crate` is a required, project-supplied plugin parameter (no
/// baked-in default) naming the crate/module root under which prost message
/// types were generated.
pub fn rust_message_path(message_crate: &str, full_name: &str) -> String {
    let mut segments: Vec<&str> = full_name.split('.').collect();
    let type_name = segments.pop().unwrap_or(full_name);
    let crate_ident = message_crate.replace('-', "_");
    let mut path = crate_ident;
    for segment in segments {
        path.push_str("::");
        path.push_str(segment);
    }
    path.push_str("::");
    path.push_str(type_name);
    path
}

/// Resolves a message's fully-qualified proto name to its Kotlin/Java
/// qualified class name, using the declaring file's `java_package` /
/// `java_multiple_files` / `java_outer_classname` options — the standard
/// protobuf-kotlin/java convention, not specific to any one project.
pub fn kotlin_message_path(file_info: &FileInfo, full_name: &str) -> String {
    let type_name = full_name.rsplit('.').next().unwrap_or(full_name);
    let package = file_info
        .java_package
        .clone()
        .unwrap_or_else(|| file_info.package.clone());

    if file_info.java_multiple_files {
        format!("{package}.{type_name}")
    } else {
        let outer = file_info
            .java_outer_classname
            .clone()
            .unwrap_or_else(|| default_outer_classname(full_name));
        format!("{package}.{outer}.{type_name}")
    }
}

/// protoc's default outer classname when `java_outer_classname` isn't set:
/// PascalCase the proto file's base name. We only need this as a fallback,
/// since every .proto this plugin expects to process should set
/// `java_multiple_files = true` explicitly (as this repo's own .proto files
/// already do).
fn default_outer_classname(full_name: &str) -> String {
    full_name
        .rsplit('.')
        .next()
        .unwrap_or(full_name)
        .to_string()
}

/// Resolves a proto package to its C#/protoc-csharp-codegen namespace: the
/// file's explicit `csharp_namespace` option if set, else each dot-segment
/// PascalCased and rejoined (protoc's own default-namespace algorithm),
/// e.g. "rosetta.example" -> "Rosetta.Example".
pub fn csharp_namespace(file_info: &FileInfo) -> String {
    file_info.csharp_namespace.clone().unwrap_or_else(|| {
        file_info
            .package
            .split('.')
            .map(|s| s.to_upper_camel_case())
            .collect::<Vec<_>>()
            .join(".")
    })
}

/// Resolves a message's fully-qualified proto name to its C# qualified
/// class name. No `java_multiple_files`/`java_outer_classname`-style
/// branching needed — protoc's C# generator always emits one top-level
/// class per message.
pub fn csharp_message_path(file_info: &FileInfo, full_name: &str) -> String {
    let type_name = full_name.rsplit('.').next().unwrap_or(full_name);
    format!("{}.{}", csharp_namespace(file_info), type_name)
}

/// Directory path (relative to the plugin's output root) that generated glue
/// for a given proto package should live under, mirroring the package the
/// way `protoc`'s own generators mirror packages for message code — e.g.
/// "rosetta.example" -> "rosetta/example".
pub fn glue_output_dir(package: &str) -> String {
    package.replace('.', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_message_path_mirrors_package() {
        assert_eq!(
            rust_message_path("protobuf_gen", "rosetta.example.CurrentTimeRequest"),
            "protobuf_gen::rosetta::example::CurrentTimeRequest"
        );
    }

    #[test]
    fn rust_message_path_top_level_package() {
        assert_eq!(
            rust_message_path("protobuf_gen", "CurrentTimeRequest"),
            "protobuf_gen::CurrentTimeRequest"
        );
    }

    #[test]
    fn kotlin_message_path_multiple_files() {
        let info = FileInfo {
            package: "rosetta.example".to_string(),
            java_package: Some("rosetta.example".to_string()),
            java_multiple_files: true,
            java_outer_classname: None,
            csharp_namespace: None,
        };
        assert_eq!(
            kotlin_message_path(&info, "rosetta.example.CurrentTimeRequest"),
            "rosetta.example.CurrentTimeRequest"
        );
    }

    #[test]
    fn kotlin_message_path_outer_classname() {
        let info = FileInfo {
            package: "rosetta.example".to_string(),
            java_package: Some("rosetta.example".to_string()),
            java_multiple_files: false,
            java_outer_classname: Some("ExampleProto".to_string()),
            csharp_namespace: None,
        };
        assert_eq!(
            kotlin_message_path(&info, "rosetta.example.CurrentTimeRequest"),
            "rosetta.example.ExampleProto.CurrentTimeRequest"
        );
    }

    #[test]
    fn csharp_namespace_defaults_to_pascal_cased_package() {
        let info = FileInfo {
            package: "rosetta.example".to_string(),
            csharp_namespace: None,
            ..Default::default()
        };
        assert_eq!(csharp_namespace(&info), "Rosetta.Example");
    }

    #[test]
    fn csharp_namespace_honors_explicit_override() {
        let info = FileInfo {
            package: "rosetta.example".to_string(),
            csharp_namespace: Some("Custom.Ns".to_string()),
            ..Default::default()
        };
        assert_eq!(csharp_namespace(&info), "Custom.Ns");
    }

    #[test]
    fn csharp_message_path_mirrors_namespace() {
        let info = FileInfo {
            package: "rosetta.example".to_string(),
            csharp_namespace: None,
            ..Default::default()
        };
        assert_eq!(
            csharp_message_path(&info, "rosetta.example.CurrentTimeRequest"),
            "Rosetta.Example.CurrentTimeRequest"
        );
    }

    #[test]
    fn glue_output_dir_mirrors_package() {
        assert_eq!(glue_output_dir("rosetta.example"), "rosetta/example");
    }
}
