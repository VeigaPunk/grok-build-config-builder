pub mod codex;
pub mod grok;
pub mod opencode;

use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductId {
    Grok,
    Codex,
    OpenCode,
}

impl ProductId {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "grok" | "grok-build" => Some(Self::Grok),
            "codex" | "codex-titanium" => Some(Self::Codex),
            "opencode" | "opencode-titanium" => Some(Self::OpenCode),
            _ => None,
        }
    }
    pub fn slug(self) -> &'static str {
        match self {
            Self::Grok => "grok",
            Self::Codex => "codex",
            Self::OpenCode => "opencode",
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldGroup {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigField {
    pub id: &'static str,
    pub path: &'static str,
    pub section: &'static str,
    pub group: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    #[serde(rename = "type")]
    pub field_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<&'static [&'static str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
}

#[derive(Clone, Serialize)]
pub struct CliFlag {
    pub flag: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

#[derive(Clone, Serialize)]
pub struct EnvVar {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

#[derive(Clone, Serialize)]
pub struct Subcommand {
    pub cmd: &'static str,
    pub desc: &'static str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preset {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub enabled: &'static [&'static str],
    pub values: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaPayload {
    pub product: &'static str,
    pub product_title: &'static str,
    pub product_tagline: &'static str,
    pub config_path: &'static str,
    pub format: &'static str, // "toml" | "json"
    pub groups: &'static [FieldGroup],
    pub fields: Vec<ConfigField>,
    pub cli_flags: &'static [CliFlag],
    pub env_vars: &'static [EnvVar],
    pub subcommands: &'static [Subcommand],
    pub presets: Vec<Preset>,
    pub version_note: &'static str,
}

pub fn jstr(s: &str) -> Value {
    Value::String(s.to_string())
}
pub fn jnum(n: f64) -> Value {
    serde_json::json!(n)
}
pub fn jbool(b: bool) -> Value {
    Value::Bool(b)
}

pub fn schema_value(id: ProductId) -> serde_json::Value {
    match id {
        ProductId::Grok => grok::value_payload(),
        ProductId::Codex => serde_json::to_value(codex::payload()).expect("codex schema"),
        ProductId::OpenCode => serde_json::to_value(opencode::payload()).expect("opencode schema"),
    }
}

pub fn generate_for(
    id: ProductId,
    enabled: &[String],
    values: &BTreeMap<String, Value>,
) -> GenerateOut {
    match id {
        ProductId::Grok => grok::generate(enabled, values),
        ProductId::Codex => codex::generate(enabled, values),
        ProductId::OpenCode => opencode::generate(enabled, values),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOut {
    pub config: String,
    pub env: String,
    pub cli: String,
    pub markdown: String,
    /// backward-compat for grok UI
    pub toml: String,
}
