//! Grok Build — thin wrappers used by generate_for; schema served via JSON merge in main if needed.

use super::*;
use crate::generate as legacy_gen;
use crate::schema as legacy;
use serde_json::Value;
use std::collections::BTreeMap;

/// Build product schema by converting legacy payload JSON + product metadata.
pub fn payload() -> SchemaPayload {
    // Use codex-like empty shell? No — merge at serialize time via value_payload()
    unreachable!("use value_payload()")
}

pub fn value_payload() -> Value {
    let p = legacy::payload();
    let mut v = serde_json::to_value(&p).expect("serialize grok schema");
    if let Some(obj) = v.as_object_mut() {
        obj.insert("product".into(), Value::String("grok".into()));
        obj.insert("productTitle".into(), Value::String("Grok Build".into()));
        obj.insert(
            "productTagline".into(),
            Value::String("Grok Build CLI config reference and patch builder.".into()),
        );
        obj.insert(
            "configPath".into(),
            Value::String("~/.grok/config.toml".into()),
        );
        obj.insert("format".into(), Value::String("toml".into()));
    }
    v
}

pub fn generate(enabled: &[String], values: &BTreeMap<String, Value>) -> GenerateOut {
    let req = legacy_gen::GenerateRequest {
        enabled: enabled.to_vec(),
        values: values.clone(),
    };
    let r = legacy_gen::generate_all(&req);
    GenerateOut {
        config: r.toml.clone(),
        toml: r.toml,
        env: r.env,
        cli: r.cli,
        markdown: r.markdown,
    }
}
