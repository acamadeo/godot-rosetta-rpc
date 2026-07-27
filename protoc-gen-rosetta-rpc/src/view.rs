//! Shared, pre-resolved template view models. All casing and type-path
//! resolution happens here in Rust, not inside the Askama templates, which
//! stay logic-free loops over already-correct strings.

#[derive(Debug, Clone)]
pub struct MethodView {
    /// The method name as written in the .proto file (e.g. "CurrentTime").
    pub proto_name: String,
    /// Language-cased method name: snake_case for Rust, camelCase for Kotlin.
    pub method_name: String,
    /// SHOUTY_SNAKE_CASE constant name for the method's descriptor.
    pub const_name: String,
    /// Fully-resolved, language-specific request type path.
    pub input_type: String,
    /// Fully-resolved, language-specific response type path.
    pub output_type: String,
}

#[derive(Debug, Clone)]
pub struct ServiceView {
    /// The service name as written in the .proto file (e.g. "Clock").
    pub proto_name: String,
    /// The proto package the service is declared in (e.g. "rosetta.example").
    pub package: String,
    /// Language-cased type name for the service interface/trait (PascalCase
    /// in both languages, so this currently just equals `proto_name`, but is
    /// kept distinct in case a future language needs different casing).
    pub type_name: String,
    /// Language-cased "factory method" / accessor name for this service:
    /// snake_case for Rust, camelCase for Kotlin (e.g. "game_service" /
    /// "gameService").
    pub factory_method_name: String,
    pub methods: Vec<MethodView>,
}
