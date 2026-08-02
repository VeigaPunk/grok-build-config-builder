//! OpenAI Codex CLI — Titanium opinionated config builder.
//! Target: ~/.codex/config.toml
//! Default preset: **Titanium** (sensible high-power defaults).

use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub fn payload() -> SchemaPayload {
    SchemaPayload {
        product: "codex",
        product_title: "Codex Titanium",
        product_tagline: "Opinionated fork of Codex CLI defaults — Titanium profile is the sensible high-power baseline.",
        config_path: "~/.codex/config.toml",
        format: "toml",
        groups: groups(),
        fields: fields(),
        cli_flags: cli_flags(),
        env_vars: env_vars(),
        subcommands: subcommands(),
        presets: presets(),
        version_note: "Codex CLI · Titanium profile · config.toml · pure Rust builder",
    }
}

fn groups() -> &'static [FieldGroup] {
    &[
        FieldGroup { id: "core", title: "Core", description: "Model, provider, personality, service tier." },
        FieldGroup { id: "reasoning", title: "Reasoning", description: "Effort, summaries, verbosity, plan-mode overrides." },
        FieldGroup { id: "sandbox", title: "Sandbox & approvals", description: "Sandbox mode, approval policy, workspace-write knobs." },
        FieldGroup { id: "permissions", title: "Permissions profile", description: "Named permission profile selection." },
        FieldGroup { id: "features", title: "Features", description: "Codex feature flags (hooks, multi-agent, memories)." },
        FieldGroup { id: "history", title: "History", description: "Session history persistence." },
        FieldGroup { id: "shell", title: "Shell environment", description: "What env vars child processes inherit." },
        FieldGroup { id: "project", title: "Project docs", description: "AGENTS.md / project markers and size caps." },
        FieldGroup { id: "provider", title: "Custom provider", description: "Optional [model_providers.*] for BYOK / proxies." },
        FieldGroup { id: "web", title: "Web search", description: "Web search tool behavior." },
    ]
}

fn fields() -> Vec<ConfigField> {
    vec![
        ConfigField { id: "model", path: "model", section: "", group: "core", label: "Model", description: "Primary model id (e.g. gpt-5.4, gpt-5.3-codex).", field_type: "string", default: Some(jstr("gpt-5.4")), options: None, env: None, cli: Some("-c model=..."), recommended: Some(true) },
        ConfigField { id: "model_provider", path: "model_provider", section: "", group: "core", label: "Model provider", description: "Built-in or custom provider id (openai, ollama, lmstudio).", field_type: "string", default: Some(jstr("openai")), options: None, env: None, cli: Some("-c model_provider=..."), recommended: Some(true) },
        ConfigField { id: "personality", path: "personality", section: "", group: "core", label: "Personality", description: "Agent tone/style when features.personality is on.", field_type: "enum", default: Some(jstr("pragmatic")), options: Some(&["pragmatic", "concise", "mentorial", "bold"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "service_tier", path: "service_tier", section: "", group: "core", label: "Service tier", description: "OpenAI service tier preference.", field_type: "enum", default: Some(jstr("flex")), options: Some(&["auto", "default", "flex", "priority"]), env: None, cli: None, recommended: None },
        ConfigField { id: "file_opener", path: "file_opener", section: "", group: "core", label: "File opener", description: "IDE used when opening files from the TUI.", field_type: "enum", default: Some(jstr("vscode")), options: Some(&["vscode", "vscode-insiders", "cursor", "windsurf", "none"]), env: None, cli: None, recommended: None },
        ConfigField { id: "model_reasoning_effort", path: "model_reasoning_effort", section: "", group: "reasoning", label: "Reasoning effort", description: "How hard the model thinks before acting.", field_type: "enum", default: Some(jstr("high")), options: Some(&["minimal", "low", "medium", "high", "xhigh"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "model_reasoning_summary", path: "model_reasoning_summary", section: "", group: "reasoning", label: "Reasoning summary", description: "Summary style for reasoning traces.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "concise", "detailed", "none"]), env: None, cli: None, recommended: None },
        ConfigField { id: "model_verbosity", path: "model_verbosity", section: "", group: "reasoning", label: "Model verbosity", description: "Output verbosity preference.", field_type: "enum", default: Some(jstr("medium")), options: Some(&["low", "medium", "high"]), env: None, cli: None, recommended: None },
        ConfigField { id: "plan_mode_reasoning_effort", path: "plan_mode_reasoning_effort", section: "", group: "reasoning", label: "Plan-mode effort", description: "Reasoning effort override inside /plan.", field_type: "enum", default: Some(jstr("xhigh")), options: Some(&["minimal", "low", "medium", "high", "xhigh"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "hide_agent_reasoning", path: "hide_agent_reasoning", section: "", group: "reasoning", label: "Hide agent reasoning", description: "Suppress reasoning events in TUI / exec.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "show_raw_agent_reasoning", path: "show_raw_agent_reasoning", section: "", group: "reasoning", label: "Show raw reasoning", description: "Surface raw reasoning when provider supports it.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "approval_policy", path: "approval_policy", section: "", group: "sandbox", label: "Approval policy", description: "untrusted | on-request | never.", field_type: "enum", default: Some(jstr("on-request")), options: Some(&["untrusted", "on-request", "never"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "sandbox_mode", path: "sandbox_mode", section: "", group: "sandbox", label: "Sandbox mode", description: "read-only | workspace-write | danger-full-access.", field_type: "enum", default: Some(jstr("workspace-write")), options: Some(&["read-only", "workspace-write", "danger-full-access"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "sandbox_workspace_write.network_access", path: "sandbox_workspace_write.network_access", section: "sandbox_workspace_write", group: "sandbox", label: "Workspace network access", description: "Allow outbound HTTP inside workspace-write sandbox.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "sandbox_workspace_write.exclude_slash_tmp", path: "sandbox_workspace_write.exclude_slash_tmp", section: "sandbox_workspace_write", group: "sandbox", label: "Exclude /tmp", description: "Drop /tmp from writable set.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "approvals_reviewer", path: "approvals_reviewer", section: "", group: "sandbox", label: "Approvals reviewer", description: "user | auto_review.", field_type: "enum", default: Some(jstr("user")), options: Some(&["user", "auto_review"]), env: None, cli: None, recommended: None },
        ConfigField { id: "default_permissions", path: "default_permissions", section: "", group: "permissions", label: "Default permissions profile", description: "Built-ins: :read-only, :workspace, :danger-full-access.", field_type: "string", default: Some(jstr(":workspace")), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "features.hooks", path: "features.hooks", section: "features", group: "features", label: "Hooks", description: "Enable lifecycle hooks.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "features.multi_agent", path: "features.multi_agent", section: "features", group: "features", label: "Multi-agent", description: "Enable multi-agent orchestration.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "features.memories", path: "features.memories", section: "features", group: "features", label: "Memories", description: "Cross-session memory feature.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.undo", path: "features.undo", section: "features", group: "features", label: "Undo", description: "Enable undo for tool side-effects when supported.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.fast_mode", path: "features.fast_mode", section: "features", group: "features", label: "Fast mode", description: "Bias toward lower latency.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.shell_tool", path: "features.shell_tool", section: "features", group: "features", label: "Shell tool", description: "Enable the shell/exec tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.personality", path: "features.personality", section: "features", group: "features", label: "Personality feature", description: "Honor the personality key.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.network_proxy", path: "features.network_proxy", section: "features", group: "features", label: "Network proxy feature", description: "Enable features.network_proxy stack.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.codex_git_commit", path: "features.codex_git_commit", section: "features", group: "features", label: "Git commit tool", description: "Enable codex git commit helper.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "history.persistence", path: "history.persistence", section: "history", group: "history", label: "History persistence", description: "save-all | none.", field_type: "enum", default: Some(jstr("save-all")), options: Some(&["save-all", "none"]), env: None, cli: None, recommended: None },
        ConfigField { id: "history.max_bytes", path: "history.max_bytes", section: "history", group: "history", label: "History max bytes", description: "Cap stored history size.", field_type: "number", default: Some(jnum(52428800.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "shell_environment_policy.inherit", path: "shell_environment_policy.inherit", section: "shell_environment_policy", group: "shell", label: "Env inherit", description: "all | core | none.", field_type: "enum", default: Some(jstr("core")), options: Some(&["all", "core", "none"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "project_doc_max_bytes", path: "project_doc_max_bytes", section: "", group: "project", label: "Project doc max bytes", description: "Max size for project instruction docs.", field_type: "number", default: Some(jnum(65536.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "web_search", path: "web_search", section: "", group: "web", label: "Web search mode", description: "cached | live | off.", field_type: "enum", default: Some(jstr("cached")), options: Some(&["cached", "live", "off"]), env: None, cli: None, recommended: None },
        ConfigField { id: "provider.id", path: "model_providers.custom.id", section: "model_providers", group: "provider", label: "Custom provider id", description: "Table name under [model_providers.id].", field_type: "string", default: Some(jstr("openai_proxy")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.name", path: "model_providers.custom.name", section: "model_providers", group: "provider", label: "Provider display name", description: "Human label for the custom provider.", field_type: "string", default: Some(jstr("OpenAI-compatible")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.base_url", path: "model_providers.custom.base_url", section: "model_providers", group: "provider", label: "Provider base URL", description: "OpenAI-compatible base URL.", field_type: "string", default: Some(jstr("https://api.openai.com/v1")), options: None, env: Some("OPENAI_BASE_URL"), cli: None, recommended: None },
        ConfigField { id: "provider.env_key", path: "model_providers.custom.env_key", section: "model_providers", group: "provider", label: "API key env var", description: "Env var holding the API key.", field_type: "string", default: Some(jstr("OPENAI_API_KEY")), options: None, env: None, cli: None, recommended: None },
    ]
}

fn cli_flags() -> &'static [CliFlag] {
    &[
        CliFlag { flag: "codex", description: "Start interactive TUI", category: "Session" },
        CliFlag { flag: "codex exec <PROMPT>", description: "Headless one-shot", category: "Session" },
        CliFlag { flag: "--profile <NAME>", description: "Apply [profiles.NAME] overlay", category: "Config" },
        CliFlag { flag: "-c / --config KEY=VALUE", description: "Override any config key", category: "Config" },
        CliFlag { flag: "-m / --model <MODEL>", description: "Model override", category: "Model" },
        CliFlag { flag: "--full-auto", description: "Autonomous mode (where supported)", category: "Permissions" },
        CliFlag { flag: "--sandbox <MODE>", description: "Sandbox override", category: "Permissions" },
        CliFlag { flag: "--cd <PATH>", description: "Working directory", category: "Session" },
        CliFlag { flag: "--json", description: "JSON event stream (exec)", category: "Headless" },
    ]
}

fn env_vars() -> &'static [EnvVar] {
    &[
        EnvVar { name: "CODEX_HOME", description: "Config/state home (default ~/.codex)", category: "Paths" },
        EnvVar { name: "OPENAI_API_KEY", description: "API key for OpenAI provider", category: "Auth" },
        EnvVar { name: "OPENAI_BASE_URL", description: "Override OpenAI-compatible base URL", category: "Auth" },
        EnvVar { name: "RUST_LOG", description: "Log filter", category: "Logging" },
    ]
}

fn subcommands() -> &'static [Subcommand] {
    &[
        Subcommand { cmd: "codex", desc: "Interactive coding agent TUI" },
        Subcommand { cmd: "codex exec", desc: "Non-interactive prompt execution" },
        Subcommand { cmd: "codex login", desc: "Authenticate" },
        Subcommand { cmd: "codex logout", desc: "Clear credentials" },
        Subcommand { cmd: "codex mcp", desc: "Manage MCP servers" },
        Subcommand { cmd: "codex completion", desc: "Shell completions" },
    ]
}

fn presets() -> Vec<Preset> {
    vec![
        Preset {
            id: "titanium",
            name: "Titanium (default)",
            description: "Sensible high-power defaults: gpt-5.4, high effort, workspace-write + network, multi-agent, hooks, memories.",
            enabled: &[
                "model", "model_provider", "personality", "service_tier", "file_opener",
                "model_reasoning_effort", "model_reasoning_summary", "model_verbosity", "plan_mode_reasoning_effort",
                "approval_policy", "sandbox_mode", "sandbox_workspace_write.network_access", "default_permissions",
                "features.hooks", "features.multi_agent", "features.memories", "features.undo",
                "features.shell_tool", "features.personality", "features.codex_git_commit",
                "history.persistence", "shell_environment_policy.inherit", "web_search",
            ],
            values: serde_json::json!({
                "model": "gpt-5.4",
                "model_provider": "openai",
                "personality": "pragmatic",
                "service_tier": "flex",
                "file_opener": "vscode",
                "model_reasoning_effort": "high",
                "model_reasoning_summary": "auto",
                "model_verbosity": "medium",
                "plan_mode_reasoning_effort": "xhigh",
                "approval_policy": "on-request",
                "sandbox_mode": "workspace-write",
                "sandbox_workspace_write.network_access": true,
                "default_permissions": ":workspace",
                "features.hooks": true,
                "features.multi_agent": true,
                "features.memories": true,
                "features.undo": true,
                "features.shell_tool": true,
                "features.personality": true,
                "features.codex_git_commit": true,
                "history.persistence": "save-all",
                "shell_environment_policy.inherit": "core",
                "web_search": "cached"
            }),
        },
        Preset {
            id: "titanium-yolo",
            name: "Titanium YOLO",
            description: "Still titanium models, but never-approve + danger-full-access. Throwaway sandboxes only.",
            enabled: &["model", "model_provider", "model_reasoning_effort", "approval_policy", "sandbox_mode", "default_permissions", "features.multi_agent", "features.hooks"],
            values: serde_json::json!({
                "model": "gpt-5.4",
                "model_provider": "openai",
                "model_reasoning_effort": "high",
                "approval_policy": "never",
                "sandbox_mode": "danger-full-access",
                "default_permissions": ":danger-full-access",
                "features.multi_agent": true,
                "features.hooks": true
            }),
        },
        Preset {
            id: "read-only-review",
            name: "Read-only review",
            description: "Safe PR review: read-only sandbox, untrusted approvals.",
            enabled: &["model", "sandbox_mode", "approval_policy", "default_permissions", "model_reasoning_effort", "features.shell_tool"],
            values: serde_json::json!({
                "model": "gpt-5.4",
                "sandbox_mode": "read-only",
                "approval_policy": "untrusted",
                "default_permissions": ":read-only",
                "model_reasoning_effort": "high",
                "features.shell_tool": true
            }),
        },
        Preset {
            id: "local-ollama",
            name: "Local Ollama",
            description: "Point Codex at a local OpenAI-compatible Ollama endpoint.",
            enabled: &["model", "model_provider", "provider.id", "provider.name", "provider.base_url", "provider.env_key", "sandbox_mode", "approval_policy"],
            values: serde_json::json!({
                "model": "gpt-oss:120b",
                "model_provider": "ollama",
                "provider.id": "ollama",
                "provider.name": "Ollama",
                "provider.base_url": "http://localhost:11434/v1",
                "provider.env_key": "OLLAMA_API_KEY",
                "sandbox_mode": "workspace-write",
                "approval_policy": "on-request"
            }),
        },
    ]
}

fn esc(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn fmt_val(field: &ConfigField, v: &Value) -> Option<String> {
    if v.is_null() { return None; }
    match field.field_type {
        "boolean" => v.as_bool().map(|b| if b { "true" } else { "false" }.into()),
        "number" => v.as_f64().map(|n| if n.fract() == 0.0 { format!("{}", n as i64) } else { format!("{}", n) }),
        _ => v.as_str().filter(|s| !s.is_empty()).map(esc),
    }
}

pub fn generate(enabled: &[String], values: &BTreeMap<String, Value>) -> GenerateOut {
    let en: BTreeSet<&str> = enabled.iter().map(|s| s.as_str()).collect();
    let all = fields();
    let mut top: Vec<String> = vec![];
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut provider_id = "custom".to_string();
    if let Some(Value::String(s)) = values.get("provider.id") { provider_id = s.clone(); }

    for f in &all {
        if !en.contains(f.id) { continue; }
        let Some(raw) = values.get(f.id) else { continue };
        let Some(formatted) = fmt_val(f, raw) else { continue };

        if f.id.starts_with("provider.") {
            if f.id == "provider.id" { continue; }
            let key = f.id.strip_prefix("provider.").unwrap();
            sections.entry(format!("model_providers.{}", provider_id)).or_default().push(format!("{} = {}", key, formatted));
            continue;
        }
        if f.section.is_empty() {
            top.push(format!("{} = {}", f.path, formatted));
        } else {
            let key = f.path.rsplit('.').next().unwrap_or(f.path);
            sections.entry(f.section.into()).or_default().push(format!("{} = {}", key, formatted));
        }
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut out = String::new();
    out.push_str("# Codex Titanium config patch\n");
    out.push_str("# Merge into ~/.codex/config.toml (or $CODEX_HOME/config.toml)\n");
    out.push_str("# Default profile philosophy: Titanium — high power, workspace-safe\n");
    out.push_str(&format!("# Generated: {}\n\n", now));
    for line in &top { out.push_str(line); out.push('\n'); }
    if !top.is_empty() { out.push('\n'); }
    let order = ["features", "sandbox_workspace_write", "history", "shell_environment_policy"];
    let mut seen = BTreeSet::new();
    for sec in order {
        if let Some(lines) = sections.get(sec) {
            seen.insert(sec.to_string());
            out.push_str(&format!("[{}]\n", sec));
            for l in lines { out.push_str(l); out.push('\n'); }
            out.push('\n');
        }
    }
    for (sec, lines) in &sections {
        if seen.contains(sec) { continue; }
        out.push_str(&format!("[{}]\n", sec));
        for l in lines { out.push_str(l); out.push('\n'); }
        out.push('\n');
    }
    out.push_str("[profiles.titanium]\n");
    out.push_str("# Activate with: codex --profile titanium\n");
    if en.contains("model") {
        if let Some(Value::String(m)) = values.get("model") { out.push_str(&format!("model = {}\n", esc(m))); }
    }
    if en.contains("model_reasoning_effort") {
        if let Some(Value::String(m)) = values.get("model_reasoning_effort") { out.push_str(&format!("model_reasoning_effort = {}\n", esc(m))); }
    }
    if en.contains("sandbox_mode") {
        if let Some(Value::String(m)) = values.get("sandbox_mode") { out.push_str(&format!("sandbox_mode = {}\n", esc(m))); }
    }
    out.push('\n');

    let mut env = format!("# Codex env overrides\n# Generated: {}\n\n", now);
    if en.contains("provider.base_url") {
        if let Some(Value::String(u)) = values.get("provider.base_url") {
            env.push_str(&format!("export OPENAI_BASE_URL={}\n", esc(u)));
        }
    }
    env.push_str("# export OPENAI_API_KEY=sk-...\n# export CODEX_HOME=~/.codex\n");

    let mut cli = format!("# Codex Titanium launch snippets\n# Generated: {}\n\n", now);
    cli.push_str("codex --profile titanium\n");
    if let Some(Value::String(m)) = values.get("model") {
        if en.contains("model") { cli.push_str(&format!("codex -c model={}\n", esc(m))); }
    }
    cli.push_str("codex exec \"review this PR for security issues\"\n");

    let md = markdown();
    GenerateOut { config: out.clone(), toml: out, env, cli, markdown: md }
}

fn markdown() -> String {
    let mut md = String::from("# Codex Titanium — Config Reference\n\n");
    md.push_str("> Opinionated **Titanium** defaults for OpenAI Codex CLI (`~/.codex/config.toml`).\n\n");
    md.push_str("## Install\n\n```bash\nnpm i -g @openai/codex\ncodex --version\n```\n\n");
    md.push_str("## Titanium default philosophy\n\n- Model: `gpt-5.4` with **high** reasoning (xhigh in plan mode)\n- Sandbox: `workspace-write` + network\n- Approvals: `on-request`\n- Features: multi-agent, hooks, memories, undo\n- Activate: `codex --profile titanium`\n\n");
    md.push_str("## Subcommands\n\n| Command | Description |\n| --- | --- |\n");
    for s in subcommands() { md.push_str(&format!("| `{}` | {} |\n", s.cmd, s.desc)); }
    md.push_str("\n## Flags\n\n| Flag | Category | Description |\n| --- | --- | --- |\n");
    for f in cli_flags() { md.push_str(&format!("| `{}` | {} | {} |\n", f.flag, f.category, f.description)); }
    md.push_str("\n## Keys\n\n");
    for g in groups() {
        md.push_str(&format!("### {}\n\n", g.title));
        for f in fields().into_iter().filter(|f| f.group == g.id) {
            md.push_str(&format!("- `{}` ({}) — {}\n", f.path, f.field_type, f.description));
        }
        md.push('\n');
    }
    md
}
