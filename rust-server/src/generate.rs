//! TOML / env / CLI patch generation from enabled fields.

use crate::schema::{fields, ConfigField};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRequest {
    pub enabled: Vec<String>,
    pub values: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateResponse {
    pub toml: String,
    pub env: String,
    pub cli: String,
    pub markdown: String,
}

fn escape_toml_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn format_value(field: &ConfigField, value: &Value) -> Option<String> {
    if value.is_null() {
        return None;
    }
    match field.field_type {
        "boolean" => value.as_bool().map(|b| if b { "true" } else { "false" }.to_string()),
        "number" => {
            if let Some(n) = value.as_f64() {
                if n.fract() == 0.0 {
                    Some(format!("{}", n as i64))
                } else {
                    Some(format!("{}", n))
                }
            } else {
                None
            }
        }
        "enum" | "string" => value.as_str().filter(|s| !s.is_empty()).map(escape_toml_string),
        "string-list" => {
            let list: Vec<String> = if let Some(arr) = value.as_array() {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .filter(|s| !s.is_empty())
                    .collect()
            } else if let Some(s) = value.as_str() {
                s.split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()
            } else {
                vec![]
            };
            if list.is_empty() {
                None
            } else {
                Some(format!(
                    "[{}]",
                    list.iter()
                        .map(|s| escape_toml_string(s))
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            }
        }
        _ => value.as_str().map(escape_toml_string),
    }
}

fn resolve_key(
    field: &ConfigField,
    values: &BTreeMap<String, Value>,
) -> Option<(String, String)> {
    if field.id.starts_with("model.custom.") {
        if field.id == "model.custom.id" {
            return None;
        }
        let model_id = values
            .get("model.custom.id")
            .and_then(|v| v.as_str())
            .unwrap_or("grok-4.5");
        let key = field.id.strip_prefix("model.custom.").unwrap_or(field.id);
        return Some((format!("model.\"{}\"", model_id), key.to_string()));
    }
    if field.id.starts_with("mcp.") {
        if field.id == "mcp.id" {
            return None;
        }
        let mcp_id = values
            .get("mcp.id")
            .and_then(|v| v.as_str())
            .unwrap_or("filesystem");
        let key = match field.id {
            "mcp.enabled" => "enabled",
            "mcp.command" => "command",
            "mcp.args" => "args",
            "mcp.startup_timeout_sec" => "startup_timeout_sec",
            "mcp.tool_timeout_sec" => "tool_timeout_sec",
            _ => return None,
        };
        return Some((format!("mcp_servers.{}", mcp_id), key.into()));
    }
    if field.path.starts_with("permissions.") {
        let key = field.path.strip_prefix("permissions.").unwrap();
        return match key {
            "permission_mode" => Some(("permissions".into(), "permission_mode".into())),
            "remember_tool_approvals" => Some(("ui".into(), "remember_tool_approvals".into())),
            "default_selected_permission" => {
                Some(("ui".into(), "default_selected_permission".into()))
            }
            _ => Some(("permissions".into(), key.into())),
        };
    }
    if field.path.starts_with("features.") {
        let key = field.path.strip_prefix("features.").unwrap();
        return Some(("features".into(), key.into()));
    }
    if field.path.starts_with("tools.") {
        let key = field.path.strip_prefix("tools.").unwrap();
        return Some(("tools".into(), key.into()));
    }
    let parts: Vec<&str> = field.path.split('.').collect();
    match parts.len() {
        2 => Some((parts[0].into(), parts[1].into())),
        3 => Some((format!("{}.{}", parts[0], parts[1]), parts[2].into())),
        4 => Some((
            format!("{}.{}.{}", parts[0], parts[1], parts[2]),
            parts[3].into(),
        )),
        _ => Some((
            field.section.into(),
            parts.last().unwrap_or(&field.id).to_string(),
        )),
    }
}

pub fn generate_toml(req: &GenerateRequest) -> String {
    let enabled: BTreeSet<&str> = req.enabled.iter().map(|s| s.as_str()).collect();
    let all = fields();
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for field in &all {
        if !enabled.contains(field.id) {
            continue;
        }
        let Some(val) = req.values.get(field.id) else {
            continue;
        };
        let Some(formatted) = format_value(field, val) else {
            continue;
        };
        let Some((section, key)) = resolve_key(field, &req.values) else {
            continue;
        };
        sections
            .entry(section)
            .or_default()
            .push(format!("{} = {}", key, formatted));
    }

    let order = [
        "models",
        "endpoints",
        "sandbox",
        "permissions",
        "session",
        "cli",
        "hints",
        "ui",
        "ui.display_refresh",
        "ui.contextual_hints",
        "tools",
        "features",
        "memory",
        "subagents",
        "skills",
        "harness",
        "telemetry",
        "auth",
        "auth.oidc",
        "grok_com_config",
        "plugins",
        "compat.cursor",
        "compat.claude",
    ];

    let mut body = String::new();
    let mut seen = BTreeSet::new();
    for sec in order {
        if let Some(lines) = sections.get(sec) {
            seen.insert(sec.to_string());
            body.push_str(&format!("[{}]\n", sec));
            for line in lines {
                body.push_str(line);
                body.push('\n');
            }
            body.push('\n');
        }
    }
    for (sec, lines) in &sections {
        if seen.contains(sec) {
            continue;
        }
        body.push_str(&format!("[{}]\n", sec));
        for line in lines {
            body.push_str(line);
            body.push('\n');
        }
        body.push('\n');
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut out = String::new();
    out.push_str("# Grok Build config patch\n");
    out.push_str("# Generated by Grok Build Config Builder (Rust)\n");
    out.push_str("# Merge into ~/.grok/config.toml (or $GROK_HOME/config.toml)\n");
    out.push_str(&format!("# Generated: {}\n\n", now));
    if body.is_empty() {
        out.push_str("# (no settings enabled — toggle fields in the builder)\n");
    } else {
        out.push_str(&body);
    }
    out
}

pub fn generate_env(req: &GenerateRequest) -> String {
    let enabled: BTreeSet<&str> = req.enabled.iter().map(|s| s.as_str()).collect();
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut lines = vec![
        "# Environment overrides for Grok Build".to_string(),
        format!("# Generated: {}", now),
        String::new(),
    ];
    let mut count = 0;
    for field in fields() {
        if !enabled.contains(field.id) {
            continue;
        }
        let Some(env) = field.env else { continue };
        let Some(raw) = req.values.get(field.id) else { continue };
        if raw.is_null() || raw.as_str() == Some("") {
            continue;
        }
        if env == "GROK_DISABLE_AUTOUPDATER" {
            if raw.as_bool() == Some(false) {
                lines.push("export GROK_DISABLE_AUTOUPDATER=1".into());
                count += 1;
            }
            continue;
        }
        let val = if let Some(b) = raw.as_bool() {
            if b { "1" } else { "0" }.to_string()
        } else if let Some(s) = raw.as_str() {
            s.to_string()
        } else if let Some(n) = raw.as_f64() {
            if n.fract() == 0.0 {
                format!("{}", n as i64)
            } else {
                format!("{}", n)
            }
        } else {
            raw.to_string()
        };
        lines.push(format!("export {}={}", env, serde_json::to_string(&val).unwrap_or(val)));
        count += 1;
    }
    if count == 0 {
        lines.push("# (no env-mapped settings enabled)".into());
    }
    lines.push(String::new());
    lines.join("\n")
}

pub fn generate_cli(req: &GenerateRequest) -> String {
    let enabled: BTreeSet<&str> = req.enabled.iter().map(|s| s.as_str()).collect();
    let mut parts = vec!["grok".to_string()];
    if enabled.contains("models.default") {
        if let Some(v) = req.values.get("models.default").and_then(|v| v.as_str()) {
            parts.push(format!("-m {}", v));
        }
    }
    if enabled.contains("models.default_reasoning_effort") {
        if let Some(v) = req
            .values
            .get("models.default_reasoning_effort")
            .and_then(|v| v.as_str())
        {
            parts.push(format!("--effort {}", v));
        }
    }
    if enabled.contains("sandbox.profile") {
        if let Some(v) = req.values.get("sandbox.profile").and_then(|v| v.as_str()) {
            parts.push(format!("--sandbox {}", v));
        }
    }
    if enabled.contains("permissions.permission_mode")
        && req
            .values
            .get("permissions.permission_mode")
            .and_then(|v| v.as_str())
            == Some("always-approve")
    {
        parts.push("--always-approve".into());
    }
    if enabled.contains("features.memory") || enabled.contains("memory.enabled") {
        let mem = req
            .values
            .get("memory.enabled")
            .or_else(|| req.values.get("features.memory"))
            .and_then(|v| v.as_bool());
        match mem {
            Some(true) => parts.push("--experimental-memory".into()),
            Some(false) => parts.push("--no-memory".into()),
            None => {}
        }
    }
    if (enabled.contains("features.subagents")
        && req.values.get("features.subagents").and_then(|v| v.as_bool()) == Some(false))
        || (enabled.contains("subagents.enabled")
            && req.values.get("subagents.enabled").and_then(|v| v.as_bool()) == Some(false))
    {
        parts.push("--no-subagents".into());
    }
    if enabled.contains("cli.auto_update")
        && req.values.get("cli.auto_update").and_then(|v| v.as_bool()) == Some(false)
    {
        parts.push("--no-auto-update".into());
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let joined = parts.join(" ");
    format!(
        "# Example launch flags derived from your selection\n# Generated: {}\n\n{}\n\n# Headless one-shot example:\n{} -p \"your prompt\" --output-format streaming-json\n",
        now, joined, joined
    )
}

pub fn generate_markdown() -> String {
    use crate::schema::{cli_flags, env_vars, groups, subcommands};
    let mut md = String::new();
    md.push_str("# Grok Build CLI — Complete Config Reference\n\n");
    md.push_str("> Generated by the **Rust** Grok Build Config Builder.\n\n");
    md.push_str("## Install\n\n```bash\ncurl -fsSL https://x.ai/cli/install.sh | bash\ngrok version\n```\n\n");
    md.push_str("## Subcommands\n\n| Command | Description |\n| --- | --- |\n");
    for s in subcommands() {
        md.push_str(&format!("| `{}` | {} |\n", s.cmd, s.desc));
    }
    md.push_str("\n## Launch flags\n\n| Flag | Category | Description |\n| --- | --- | --- |\n");
    for f in cli_flags() {
        md.push_str(&format!("| `{}` | {} | {} |\n", f.flag, f.category, f.description));
    }
    md.push_str("\n## Environment variables\n\n| Variable | Category | Description |\n| --- | --- | --- |\n");
    for e in env_vars() {
        md.push_str(&format!("| `{}` | {} | {} |\n", e.name, e.category, e.description));
    }
    md.push_str("\n## config.toml keys\n\n");
    for g in groups() {
        md.push_str(&format!("### {}\n\n{}\n\n", g.title, g.description));
        md.push_str("| Key | Type | Default | Env | CLI | Description |\n| --- | --- | --- | --- | --- | --- |\n");
        for f in fields().into_iter().filter(|f| f.group == g.id) {
            let def = f
                .default
                .as_ref()
                .map(|v| format!("`{}`", v))
                .unwrap_or_else(|| "—".into());
            let env = f.env.unwrap_or("—");
            let cli = f.cli.unwrap_or("—");
            md.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | {} |\n",
                f.path, f.field_type, def, env, cli, f.description
            ));
        }
        md.push('\n');
    }
    md
}

pub fn generate_all(req: &GenerateRequest) -> GenerateResponse {
    GenerateResponse {
        toml: generate_toml(req),
        env: generate_env(req),
        cli: generate_cli(req),
        markdown: generate_markdown(),
    }
}
