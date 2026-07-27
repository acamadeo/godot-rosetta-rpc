//! Extracts a language-agnostic intermediate representation (IR) of every RPC
//! service in the request, plus a resolver for mapping proto message type
//! names to per-language qualified paths.

use std::collections::HashMap;

use prost_types::compiler::CodeGeneratorRequest;
use prost_types::{DescriptorProto, EnumDescriptorProto, FileDescriptorProto};

#[derive(Debug, Clone)]
pub struct TypeRef {
    /// Fully-qualified proto type name, without a leading dot (e.g.
    /// "rosetta.example.CurrentTimeRequest").
    pub full_name: String,
}

#[derive(Debug, Clone)]
pub struct MethodIr {
    /// The method name as written in the .proto file (e.g. "CurrentTime").
    pub name: String,
    pub input: TypeRef,
    pub output: TypeRef,
}

#[derive(Debug, Clone)]
pub struct ServiceIr {
    /// The service name as written in the .proto file (e.g. "Clock").
    pub name: String,
    /// The proto package the service is declared in (e.g. "rosetta.example").
    pub package: String,
    pub methods: Vec<MethodIr>,
}

/// Per-file information needed to resolve a message type name to a
/// language-specific qualified path.
#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub package: String,
    pub java_package: Option<String>,
    pub java_multiple_files: bool,
    pub java_outer_classname: Option<String>,
}

/// Maps every message/enum full proto name (no leading dot) encountered
/// anywhere in the request (including transitive dependencies) to the
/// [`FileInfo`] of the file that declared it.
#[derive(Debug, Default)]
pub struct TypeResolver {
    file_info_by_type: HashMap<String, FileInfo>,
}

impl TypeResolver {
    pub fn build(proto_files: &[FileDescriptorProto]) -> Self {
        let mut file_info_by_type = HashMap::new();
        for file in proto_files {
            let package = file.package.clone().unwrap_or_default();
            let options = file.options.as_ref();
            let info = FileInfo {
                package: package.clone(),
                java_package: options.and_then(|o| o.java_package.clone()),
                java_multiple_files: options.and_then(|o| o.java_multiple_files).unwrap_or(false),
                java_outer_classname: options.and_then(|o| o.java_outer_classname.clone()),
            };
            for message in &file.message_type {
                collect_message_types(message, &package, &info, &mut file_info_by_type);
            }
            for enum_type in &file.enum_type {
                collect_enum_type(enum_type, &package, &info, &mut file_info_by_type);
            }
        }
        Self { file_info_by_type }
    }

    pub fn resolve(&self, full_name: &str) -> Option<&FileInfo> {
        self.file_info_by_type.get(full_name)
    }
}

fn collect_message_types(
    message: &DescriptorProto,
    parent_package: &str,
    info: &FileInfo,
    out: &mut HashMap<String, FileInfo>,
) {
    let Some(name) = message.name.as_deref() else {
        return;
    };
    let full_name = qualify(parent_package, name);
    for nested in &message.nested_type {
        collect_message_types(nested, &full_name, info, out);
    }
    for nested_enum in &message.enum_type {
        collect_enum_type(nested_enum, &full_name, info, out);
    }
    out.insert(full_name, info.clone());
}

fn collect_enum_type(
    enum_type: &EnumDescriptorProto,
    parent_package: &str,
    info: &FileInfo,
    out: &mut HashMap<String, FileInfo>,
) {
    let Some(name) = enum_type.name.as_deref() else {
        return;
    };
    out.insert(qualify(parent_package, name), info.clone());
}

fn qualify(package: &str, name: &str) -> String {
    if package.is_empty() {
        name.to_string()
    } else {
        format!("{package}.{name}")
    }
}

fn strip_leading_dot(name: &str) -> String {
    name.strip_prefix('.').unwrap_or(name).to_string()
}

pub struct Ir {
    pub services: Vec<ServiceIr>,
    pub resolver: TypeResolver,
}

/// Extracts the IR for every service declared directly in one of
/// `request.file_to_generate` (not its transitive imports).
///
/// Also builds a [`TypeResolver`] covering every file protoc handed us
/// (including dependencies), so imported message types still resolve.
pub fn extract(request: &CodeGeneratorRequest) -> Ir {
    let resolver = TypeResolver::build(&request.proto_file);

    let mut services = Vec::new();
    for file in &request.proto_file {
        let Some(file_name) = file.name.as_deref() else {
            continue;
        };
        if !request.file_to_generate.iter().any(|f| f == file_name) {
            continue;
        }
        let package = file.package.clone().unwrap_or_default();
        for service in &file.service {
            let Some(service_name) = service.name.as_deref() else {
                continue;
            };
            let methods = service
                .method
                .iter()
                .filter_map(|method| {
                    let name = method.name.as_deref()?;
                    let input = method.input_type.as_deref()?;
                    let output = method.output_type.as_deref()?;
                    Some(MethodIr {
                        name: name.to_string(),
                        input: TypeRef {
                            full_name: strip_leading_dot(input),
                        },
                        output: TypeRef {
                            full_name: strip_leading_dot(output),
                        },
                    })
                })
                .collect();
            services.push(ServiceIr {
                name: service_name.to_string(),
                package: package.clone(),
                methods,
            });
        }
    }

    Ir { services, resolver }
}
