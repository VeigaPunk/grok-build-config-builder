//! OpenCode Titanium — wild but coherent config builder.
//! Target: opencode.json / ~/.config/opencode/opencode.json

use super::*;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub fn payload() -> SchemaPayload {
    SchemaPayload {
        product: "opencode",
        product_title: "OpenCode Titanium",
        product_tagline: "Titanium-grade OpenCode build — agents, MCP, plan mode, subagents, and sharp defaults.",
        config_path: "~/.config/opencode/opencode.json",
        format: "json",
        groups: groups(),
        fields: fields(),
        cli_flags: cli_flags(),
        env_vars: env_vars(),
        subcommands: subcommands(),
        presets: presets(),
        version_note: "OpenCode · Titanium build · opencode.json · pure Rust builder",
    }
}

fn groups() -> &'static [FieldGroup] {
    &[
        FieldGroup { id: "models", title: "Models", description: "Primary + small models and provider allowlists." },
        FieldGroup { id: "agents", title: "Agents", description: "Default agent, depth, plan mode." },
        FieldGroup { id: "tools", title: "Tools & permissions", description: "write/bash/edit gates and tool toggles." },
        FieldGroup { id: "compaction", title: "Compaction", description: "Context auto-compact settings." },
        FieldGroup { id: "server", title: "Server", description: "opencode serve / web." },
        FieldGroup { id: "ux", title: "UX", description: "Theme, share, autoupdate, snapshot, shell." },
        FieldGroup { id: "experimental", title: "Experimental", description: "Plan mode, background subagents, scout, workspaces." },
        FieldGroup { id: "lsp", title: "LSP & formatters", description: "Language intelligence and format-on-write." },
        FieldGroup { id: "mcp", title: "MCP sample", description: "Optional remote MCP server stub." },
    ]
}

fn fields() -> Vec<ConfigField> {
    vec![
        ConfigField { id: "model", path: "model", section: "", group: "models", label: "Primary model", description: "provider/model id, e.g. anthropic/claude-sonnet-4-6 or openai/gpt-5.4.", field_type: "string", default: Some(jstr("anthropic/claude-sonnet-4-6")), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "small_model", path: "small_model", section: "", group: "models", label: "Small model", description: "Cheap model for titles / light tasks.", field_type: "string", default: Some(jstr("anthropic/claude-haiku-4-5")), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "enabled_providers", path: "enabled_providers", section: "", group: "models", label: "Enabled providers", description: "Comma-separated allow-list of provider ids.", field_type: "string-list", default: Some(json!(["anthropic", "openai", "openrouter"])), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "disabled_providers", path: "disabled_providers", section: "", group: "models", label: "Disabled providers", description: "Comma-separated deny-list.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "default_agent", path: "default_agent", section: "", group: "agents", label: "Default agent", description: "build | plan | or custom agent id.", field_type: "string", default: Some(jstr("build")), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "subagent_depth", path: "subagent_depth", section: "", group: "agents", label: "Subagent depth", description: "Max nesting (0 disables).", field_type: "number", default: Some(jnum(2.0)), options: None, env: None, cli: None, recommended: Some(true) },

        ConfigField { id: "tools.write", path: "tools.write", section: "tools", group: "tools", label: "Write tool", description: "Allow write tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.bash", path: "tools.bash", section: "tools", group: "tools", label: "Bash tool", description: "Allow bash tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "edit", path: "edit", section: "", group: "tools", label: "Edit permission", description: "allow | ask | deny for edits.", field_type: "enum", default: Some(jstr("allow")), options: Some(&["allow", "ask", "deny"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "bash", path: "bash", section: "", group: "tools", label: "Bash permission", description: "allow | ask | deny for bash.", field_type: "enum", default: Some(jstr("ask")), options: Some(&["allow", "ask", "deny"]), env: Some("OPENCODE_PERMISSION"), cli: None, recommended: Some(true) },

        ConfigField { id: "compaction.auto", path: "compaction.auto", section: "compaction", group: "compaction", label: "Auto compact", description: "Automatically compact long contexts.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_DISABLE_AUTOCOMPACT"), cli: None, recommended: Some(true) },
        ConfigField { id: "compaction.prune", path: "compaction.prune", section: "compaction", group: "compaction", label: "Prune on compact", description: "Prune old turns when compacting.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "compaction.reserved", path: "compaction.reserved", section: "compaction", group: "compaction", label: "Reserved tokens", description: "Token buffer reserved during compact.", field_type: "number", default: Some(jnum(12_000.0)), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "server.port", path: "server.port", section: "server", group: "server", label: "Server port", description: "Port for opencode serve / web.", field_type: "number", default: Some(jnum(4096.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "server.hostname", path: "server.hostname", section: "server", group: "server", label: "Server hostname", description: "Bind host.", field_type: "string", default: Some(jstr("127.0.0.1")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "server.mdns", path: "server.mdns", section: "server", group: "server", label: "mDNS", description: "Advertise via mDNS.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "theme", path: "theme", section: "", group: "ux", label: "Theme", description: "TUI theme (system or named).", field_type: "string", default: Some(jstr("system")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "share", path: "share", section: "", group: "ux", label: "Share mode", description: "manual | auto | disabled.", field_type: "enum", default: Some(jstr("manual")), options: Some(&["manual", "auto", "disabled"]), env: Some("OPENCODE_AUTO_SHARE"), cli: None, recommended: None },
        ConfigField { id: "autoupdate", path: "autoupdate", section: "", group: "ux", label: "Autoupdate", description: "true | false | notify.", field_type: "enum", default: Some(jstr("notify")), options: Some(&["true", "false", "notify"]), env: Some("OPENCODE_DISABLE_AUTOUPDATE"), cli: None, recommended: None },
        ConfigField { id: "snapshot", path: "snapshot", section: "", group: "ux", label: "Snapshot tracking", description: "Track file changes during session.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "shell", path: "shell", section: "", group: "ux", label: "Shell", description: "Shell binary for tools (empty = auto).", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "username", path: "username", section: "", group: "ux", label: "Username", description: "Display name in TUI.", field_type: "string", default: Some(jstr("titanium")), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "experimental.plan_mode", path: "experimental.plan_mode", section: "experimental", group: "experimental", label: "Plan mode", description: "Enable experimental plan mode.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_PLAN_MODE"), cli: None, recommended: Some(true) },
        ConfigField { id: "experimental.background_subagents", path: "experimental.background_subagents", section: "experimental", group: "experimental", label: "Background subagents", description: "Run subagents in background.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_BACKGROUND_SUBAGENTS"), cli: None, recommended: Some(true) },
        ConfigField { id: "experimental.scout", path: "experimental.scout", section: "experimental", group: "experimental", label: "Scout subagent", description: "Enable Scout research subagent.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_SCOUT"), cli: None, recommended: Some(true) },
        ConfigField { id: "experimental.workspaces", path: "experimental.workspaces", section: "experimental", group: "experimental", label: "Workspaces", description: "Enable workspace support.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_WORKSPACES"), cli: None, recommended: None },
        ConfigField { id: "experimental.hooks", path: "experimental.hooks", section: "experimental", group: "experimental", label: "Hooks", description: "Experimental hooks system.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "experimental.lsp_tool", path: "experimental.lsp_tool", section: "experimental", group: "experimental", label: "LSP tool", description: "Expose LSP as a tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_LSP_TOOL"), cli: None, recommended: None },
        ConfigField { id: "experimental.parallel", path: "experimental.parallel", section: "experimental", group: "experimental", label: "Parallel search", description: "Parallel web search execution.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_PARALLEL"), cli: None, recommended: None },

        ConfigField { id: "lsp", path: "lsp", section: "", group: "lsp", label: "LSP enabled", description: "Enable language servers.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_DISABLE_LSP_DOWNLOAD"), cli: None, recommended: Some(true) },
        ConfigField { id: "formatter", path: "formatter", section: "", group: "lsp", label: "Formatters enabled", description: "Enable code formatters.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "mcp.demo.enabled", path: "mcp.demo.enabled", section: "mcp", group: "mcp", label: "Demo MCP enabled", description: "Include a sample remote MCP entry.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.demo.url", path: "mcp.demo.url", section: "mcp", group: "mcp", label: "Demo MCP URL", description: "Remote MCP server URL.", field_type: "string", default: Some(jstr("https://mcp.example.com/sse")), options: None, env: None, cli: None, recommended: None },
    ]
}

fn cli_flags() -> &'static [CliFlag] {
    &[
        CliFlag { flag: "opencode", description: "Start TUI", category: "Session" },
        CliFlag { flag: "opencode run \"<prompt>\"", description: "Headless run", category: "Session" },
        CliFlag { flag: "opencode serve", description: "Headless API server", category: "Server" },
        CliFlag { flag: "opencode web", description: "Server + web UI", category: "Server" },
        CliFlag { flag: "--pure", description: "No external plugins", category: "Global" },
        CliFlag { flag: "--log-level <LEVEL>", description: "DEBUG|INFO|WARN|ERROR", category: "Global" },
        CliFlag { flag: "--print-logs", description: "Logs to stderr", category: "Global" },
        CliFlag { flag: "opencode mcp add|list|auth", description: "MCP management", category: "MCP" },
        CliFlag { flag: "opencode models", description: "List models", category: "Models" },
        CliFlag { flag: "opencode agent create|list", description: "Agents", category: "Agents" },
    ]
}

fn env_vars() -> &'static [EnvVar] {
    &[
        EnvVar { name: "OPENCODE_CONFIG", description: "Path to config file", category: "Paths" },
        EnvVar { name: "OPENCODE_CONFIG_DIR", description: "Config directory", category: "Paths" },
        EnvVar { name: "OPENCODE_CONFIG_CONTENT", description: "Inline JSON overrides", category: "Paths" },
        EnvVar { name: "OPENCODE_DISABLE_AUTOUPDATE", description: "Disable update checks", category: "UX" },
        EnvVar { name: "OPENCODE_DISABLE_AUTOCOMPACT", description: "Disable auto compact", category: "Context" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL", description: "Umbrella experimental flag", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_PLAN_MODE", description: "Plan mode", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_BACKGROUND_SUBAGENTS", description: "Background subagents", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_SCOUT", description: "Scout subagent", category: "Experimental" },
        EnvVar { name: "OPENCODE_SERVER_PASSWORD", description: "Basic auth for serve/web", category: "Server" },
    ]
}

fn subcommands() -> &'static [Subcommand] {
    &[
        Subcommand { cmd: "opencode", desc: "Interactive TUI" },
        Subcommand { cmd: "opencode run", desc: "Non-interactive prompt" },
        Subcommand { cmd: "opencode serve", desc: "Headless server" },
        Subcommand { cmd: "opencode web", desc: "Web UI server" },
        Subcommand { cmd: "opencode mcp", desc: "MCP servers" },
        Subcommand { cmd: "opencode models", desc: "List models" },
        Subcommand { cmd: "opencode agent", desc: "Manage agents" },
        Subcommand { cmd: "opencode session", desc: "Sessions" },
        Subcommand { cmd: "opencode upgrade", desc: "Self-update" },
    ]
}

fn presets() -> Vec<Preset> {
    vec![
        Preset {
            id: "titanium",
            name: "Titanium (default)",
            description: "Wild-but-sane: Sonnet primary, Haiku small, plan mode, scout, background subagents, ask-before-bash, depth 2.",
            enabled: &[
                "model", "small_model", "enabled_providers", "default_agent", "subagent_depth",
                "tools.write", "tools.bash", "edit", "bash",
                "compaction.auto", "compaction.prune", "compaction.reserved",
                "theme", "share", "autoupdate", "snapshot", "username",
                "experimental.plan_mode", "experimental.background_subagents", "experimental.scout",
                "experimental.workspaces", "experimental.hooks", "experimental.lsp_tool", "experimental.parallel",
                "lsp", "formatter",
            ],
            values: serde_json::json!({
                "model": "anthropic/claude-sonnet-4-6",
                "small_model": "anthropic/claude-haiku-4-5",
                "enabled_providers": ["anthropic", "openai", "openrouter"],
                "default_agent": "build",
                "subagent_depth": 2,
                "tools.write": true,
                "tools.bash": true,
                "edit": "allow",
                "bash": "ask",
                "compaction.auto": true,
                "compaction.prune": true,
                "compaction.reserved": 12000,
                "theme": "system",
                "share": "manual",
                "autoupdate": "notify",
                "snapshot": true,
                "username": "titanium",
                "experimental.plan_mode": true,
                "experimental.background_subagents": true,
                "experimental.scout": true,
                "experimental.workspaces": true,
                "experimental.hooks": true,
                "experimental.lsp_tool": true,
                "experimental.parallel": true,
                "lsp": true,
                "formatter": true
            }),
        },
        Preset {
            id: "titanium-max",
            name: "Titanium Max",
            description: "Everything on: deeper subagents, auto-share off, server bind, demo MCP stub, parallel + scout.",
            enabled: &[
                "model", "small_model", "default_agent", "subagent_depth",
                "tools.write", "tools.bash", "edit", "bash",
                "compaction.auto", "server.port", "server.hostname",
                "experimental.plan_mode", "experimental.background_subagents", "experimental.scout",
                "experimental.workspaces", "experimental.hooks", "experimental.lsp_tool", "experimental.parallel",
                "lsp", "formatter", "mcp.demo.enabled", "mcp.demo.url", "username", "share",
            ],
            values: serde_json::json!({
                "model": "anthropic/claude-opus-4-5",
                "small_model": "anthropic/claude-haiku-4-5",
                "default_agent": "build",
                "subagent_depth": 3,
                "tools.write": true,
                "tools.bash": true,
                "edit": "allow",
                "bash": "allow",
                "compaction.auto": true,
                "server.port": 4096,
                "server.hostname": "127.0.0.1",
                "experimental.plan_mode": true,
                "experimental.background_subagents": true,
                "experimental.scout": true,
                "experimental.workspaces": true,
                "experimental.hooks": true,
                "experimental.lsp_tool": true,
                "experimental.parallel": true,
                "lsp": true,
                "formatter": true,
                "mcp.demo.enabled": false,
                "mcp.demo.url": "https://mcp.example.com/sse",
                "username": "titanium-max",
                "share": "disabled"
            }),
        },
        Preset {
            id: "safe-pair",
            name: "Safe pair-programming",
            description: "Ask on edit+bash, no background agents, shallow depth.",
            enabled: &["model", "small_model", "edit", "bash", "tools.write", "tools.bash", "subagent_depth", "experimental.plan_mode", "share"],
            values: serde_json::json!({
                "model": "anthropic/claude-sonnet-4-6",
                "small_model": "anthropic/claude-haiku-4-5",
                "edit": "ask",
                "bash": "ask",
                "tools.write": true,
                "tools.bash": true,
                "subagent_depth": 1,
                "experimental.plan_mode": true,
                "share": "disabled"
            }),
        },
        Preset {
            id: "openai-codex-bridge",
            name: "OpenAI bridge",
            description: "Route primary model through OpenAI gpt-5.4 while keeping Titanium UX.",
            enabled: &["model", "small_model", "enabled_providers", "default_agent", "subagent_depth", "edit", "bash", "experimental.plan_mode", "lsp"],
            values: serde_json::json!({
                "model": "openai/gpt-5.4",
                "small_model": "openai/gpt-5-mini",
                "enabled_providers": ["openai"],
                "default_agent": "build",
                "subagent_depth": 2,
                "edit": "allow",
                "bash": "ask",
                "experimental.plan_mode": true,
                "lsp": true
            }),
        },
    ]
}

fn set_path(root: &mut Map<String, Value>, path: &str, value: Value) {
    let parts: Vec<&str> = path.split('.').collect();
    fn insert(map: &mut Map<String, Value>, parts: &[&str], value: Value) {
        if parts.len() == 1 {
            map.insert(parts[0].to_string(), value);
            return;
        }
        let entry = map
            .entry(parts[0].to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if !entry.is_object() {
            *entry = Value::Object(Map::new());
        }
        insert(entry.as_object_mut().unwrap(), &parts[1..], value);
    }
    insert(root, &parts, value);
}

fn coerce(field: &ConfigField, v: &Value) -> Option<Value> {
    if v.is_null() {
        return None;
    }
    match field.field_type {
        "boolean" => v.as_bool().map(Value::Bool),
        "number" => v.as_f64().map(|n| {
            if n.fract() == 0.0 {
                json!(n as i64)
            } else {
                json!(n)
            }
        }),
        "string-list" => {
            if let Some(arr) = v.as_array() {
                Some(Value::Array(arr.clone()))
            } else if let Some(s) = v.as_str() {
                let list: Vec<Value> = s
                    .split(',')
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty())
                    .map(|x| Value::String(x.to_string()))
                    .collect();
                if list.is_empty() {
                    None
                } else {
                    Some(Value::Array(list))
                }
            } else {
                None
            }
        }
        "enum" | "string" => {
            // autoupdate true/false as bool-ish strings stay strings; special-case
            if field.id == "autoupdate" {
                return v.as_str().map(|s| match s {
                    "true" => Value::Bool(true),
                    "false" => Value::Bool(false),
                    other => Value::String(other.to_string()),
                });
            }
            v.as_str()
                .filter(|s| !s.is_empty())
                .map(|s| Value::String(s.to_string()))
        }
        _ => Some(v.clone()),
    }
}

pub fn generate(enabled: &[String], values: &BTreeMap<String, Value>) -> GenerateOut {
    let en: BTreeSet<&str> = enabled.iter().map(|s| s.as_str()).collect();
    let mut root = Map::new();
    root.insert(
        "$schema".into(),
        Value::String("https://opencode.ai/config.json".into()),
    );
    root.insert(
        "_comment".into(),
        Value::String("OpenCode Titanium build — generated patch".into()),
    );

    // agent stubs for titanium
    if en.contains("default_agent") {
        let mut agents = Map::new();
        agents.insert(
            "build".into(),
            json!({
                "description": "Titanium build agent — implement and verify",
                "prompt": "You are OpenCode Titanium build. Ship working code, verify with tests, prefer small diffs."
            }),
        );
        agents.insert(
            "plan".into(),
            json!({
                "description": "Titanium plan agent — designs before edits",
                "prompt": "You are OpenCode Titanium plan. Explore, outline steps, identify risks, do not edit until asked."
            }),
        );
        agents.insert(
            "scout".into(),
            json!({
                "description": "Research/scout subagent",
                "prompt": "Search the codebase and web. Return concise findings with file paths."
            }),
        );
        root.insert("agent".into(), Value::Object(agents));
    }

    for f in fields() {
        if !en.contains(f.id) {
            continue;
        }
        // mcp special
        if f.id == "mcp.demo.enabled" {
            continue;
        }
        if f.id == "mcp.demo.url" {
            if en.contains("mcp.demo.enabled") && values.get("mcp.demo.enabled").and_then(|v| v.as_bool()) == Some(true) {
                let url = values
                    .get("mcp.demo.url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("https://mcp.example.com/sse");
                let mut mcp = Map::new();
                mcp.insert(
                    "demo".into(),
                    json!({ "type": "remote", "url": url, "enabled": true }),
                );
                root.insert("mcp".into(), Value::Object(mcp));
            }
            continue;
        }
        let Some(raw) = values.get(f.id) else { continue };
        let Some(val) = coerce(&f, raw) else { continue };
        set_path(&mut root, f.path, val);
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let config = serde_json::to_string_pretty(&Value::Object(root)).unwrap_or_else(|_| "{}".into());
    let header = format!(
        "// OpenCode Titanium config\n// Merge into ~/.config/opencode/opencode.json or project opencode.json\n// Generated: {}\n",
        now
    );
    // JSON doesn't support comments in strict parsers — provide pure JSON as config
    let _ = header;
    let config_out = config;

    let mut env = format!("# OpenCode Titanium env\n# Generated: {}\n\n", now);
    env.push_str("export OPENCODE_EXPERIMENTAL=1\n");
    if en.contains("experimental.plan_mode")
        && values
            .get("experimental.plan_mode")
            .and_then(|v| v.as_bool())
            == Some(true)
    {
        env.push_str("export OPENCODE_EXPERIMENTAL_PLAN_MODE=1\n");
    }
    if en.contains("experimental.background_subagents")
        && values
            .get("experimental.background_subagents")
            .and_then(|v| v.as_bool())
            == Some(true)
    {
        env.push_str("export OPENCODE_EXPERIMENTAL_BACKGROUND_SUBAGENTS=1\n");
    }
    if en.contains("experimental.scout")
        && values.get("experimental.scout").and_then(|v| v.as_bool()) == Some(true)
    {
        env.push_str("export OPENCODE_EXPERIMENTAL_SCOUT=1\n");
    }
    env.push_str("# export OPENCODE_CONFIG=~/.config/opencode/opencode.json\n");

    let mut cli = format!("# OpenCode Titanium launches\n# Generated: {}\n\n", now);
    cli.push_str("opencode\n");
    cli.push_str("opencode run \"implement the next failing test\"\n");
    cli.push_str("opencode serve\n");
    cli.push_str("opencode web\n");

    GenerateOut {
        config: config_out.clone(),
        toml: config_out,
        env,
        cli,
        markdown: markdown(),
    }
}

fn markdown() -> String {
    let mut md = String::from("# OpenCode Titanium — Config Reference\n\n");
    md.push_str("> **Titanium build** for [OpenCode](https://opencode.ai) — agents, plan mode, scout, subagents.\n\n");
    md.push_str("## Install\n\n```bash\ncurl -fsSL https://opencode.ai/install | bash\nopencode --version\n```\n\n");
    md.push_str("## Config locations\n\n1. `~/.config/opencode/opencode.json` (global)\n2. `opencode.json` in project root\n3. `$OPENCODE_CONFIG` / `$OPENCODE_CONFIG_CONTENT`\n\n");
    md.push_str("## Titanium defaults\n\n- Primary: Claude Sonnet 4.6 · Small: Haiku 4.5\n- `default_agent = build` with plan + scout agent stubs\n- `subagent_depth = 2`, plan mode + background subagents + scout\n- bash = ask, edit = allow, LSP + formatters on\n\n");
    md.push_str("## Commands\n\n| Command | Description |\n| --- | --- |\n");
    for s in subcommands() {
        md.push_str(&format!("| `{}` | {} |\n", s.cmd, s.desc));
    }
    md.push_str("\n## Flags & env\n\n| Flag | Description |\n| --- | --- |\n");
    for f in cli_flags() {
        md.push_str(&format!("| `{}` | {} |\n", f.flag, f.description));
    }
    md.push_str("\n| Env | Description |\n| --- | --- |\n");
    for e in env_vars() {
        md.push_str(&format!("| `{}` | {} |\n", e.name, e.description));
    }
    md.push_str("\n## Keys\n\n");
    for g in groups() {
        md.push_str(&format!("### {}\n\n", g.title));
        for f in fields().into_iter().filter(|x| x.group == g.id) {
            md.push_str(&format!("- `{}` ({}) — {}\n", f.path, f.field_type, f.description));
        }
        md.push('\n');
    }
    md
}
