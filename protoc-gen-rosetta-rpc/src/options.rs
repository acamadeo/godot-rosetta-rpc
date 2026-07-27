//! Parses the `parameter` string protoc passes through from
//! `--rosetta-rpc_out=<parameter>:<out_dir>`, following the same
//! `key=value,key=value` convention used by `protoc-gen-prost` and most other
//! protoc plugins.

use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Lang {
    Rust,
    Kotlin,
}

impl Lang {
    fn as_str(self) -> &'static str {
        match self {
            Lang::Rust => "rust",
            Lang::Kotlin => "kotlin",
        }
    }

    fn parse(raw: &str) -> Option<Lang> {
        Lang::iter().find(|lang| lang.as_str() == raw)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    /// The language to generate bindings for.
    pub lang: Lang,
    /// Required when `lang` is Rust: the crate/module root under which
    /// prost-generated message types live (e.g. "protobuf_gen"). No default —
    /// this plugin has no opinion on a specific project's crate layout.
    pub message_crate: Option<String>,
}

impl Options {
    pub fn parse(raw: &str) -> Result<Self, String> {
        let mut lang: Option<Lang> = None;
        let mut message_crate: Option<String> = None;

        for entry in raw.split(',') {
            if entry.is_empty() {
                continue;
            }
            let mut parts = entry.splitn(2, '=');
            let key = parts.next().unwrap_or_default();
            let value = parts.next();
            match key {
                "lang" => {
                    let v = value.ok_or_else(|| "lang parameter requires a value".to_string())?;
                    lang = Some(Lang::parse(v).ok_or_else(|| format!("unsupported lang: {v}"))?);
                }
                "message_crate" => {
                    message_crate = value.map(|v| v.to_string());
                }
                other => return Err(format!("unknown plugin parameter: {other}")),
            }
        }

        let lang = match lang {
            Some(lang) => lang,
            None => {
                let choices = Lang::iter().map(Lang::as_str).collect::<Vec<_>>().join("|");
                return Err(format!("missing required parameter: lang={choices}"));
            }
        };
        if lang == Lang::Rust && message_crate.is_none() {
            return Err(
                "missing required parameter for lang=rust: message_crate (the crate holding \
                 your prost-generated message types)"
                    .to_string(),
            );
        }

        Ok(Options {
            lang,
            message_crate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rust_params() {
        let options = Options::parse("lang=rust,message_crate=protobuf_gen").unwrap();
        assert_eq!(options.lang, Lang::Rust);
        assert_eq!(options.message_crate.as_deref(), Some("protobuf_gen"));
    }

    #[test]
    fn parses_kotlin_params_without_message_crate() {
        let options = Options::parse("lang=kotlin").unwrap();
        assert_eq!(options.lang, Lang::Kotlin);
        assert_eq!(options.message_crate, None);
    }

    #[test]
    fn last_repeated_lang_wins() {
        let options = Options::parse("lang=rust,lang=kotlin,message_crate=protobuf_gen").unwrap();
        assert_eq!(options.lang, Lang::Kotlin);
    }

    #[test]
    fn rejects_missing_lang() {
        assert!(Options::parse("message_crate=protobuf_gen").is_err());
    }

    #[test]
    fn rejects_rust_without_message_crate() {
        assert!(Options::parse("lang=rust").is_err());
    }

    #[test]
    fn rejects_unknown_key() {
        assert!(Options::parse("lang=rust,message_crate=x,bogus=1").is_err());
    }
}
