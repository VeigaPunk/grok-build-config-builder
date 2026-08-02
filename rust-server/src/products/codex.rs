//! OpenAI Codex CLI — Titanium opinionated config builder.
//! Target: ~/.codex/config.toml
//! Exhaustive knobs from official Configuration Reference + docs.

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
        version_note: "Codex CLI · Titanium profile · config.toml · pure Rust builder · exhaustive knobs from official reference",
    }
}

fn groups() -> &'static [FieldGroup] {
    &[
        FieldGroup { id: "core", title: "Core", description: "Model, provider, personality, service tier, review model, compact." },
        FieldGroup { id: "reasoning", title: "Reasoning", description: "Effort, summaries, verbosity, plan-mode overrides." },
        FieldGroup { id: "sandbox", title: "Sandbox & approvals", description: "Sandbox mode, approval policy, workspace-write knobs, granular approvals." },
        FieldGroup { id: "permissions", title: "Permissions profile", description: "Named permission profile selection." },
        FieldGroup { id: "agents", title: "Multi-agent", description: "Spawned agents, concurrency, default subagent model." },
        FieldGroup { id: "features", title: "Features", description: "Codex feature flags (hooks, multi-agent, memories, apps, web search)." },
        FieldGroup { id: "history", title: "History", description: "Session history persistence." },
        FieldGroup { id: "shell", title: "Shell environment", description: "What env vars child processes inherit." },
        FieldGroup { id: "project", title: "Project docs", description: "AGENTS.md / project markers and size caps." },
        FieldGroup { id: "provider", title: "Custom provider", description: "Optional [model_providers.*] for BYOK / proxies." },
        FieldGroup { id: "web", title: "Web search", description: "Web search tool behavior." },
        FieldGroup { id: "mcp", title: "MCP servers", description: "Sample [mcp_servers.*] entry for tools." },
        FieldGroup { id: "memories", title: "Memories", description: "Cross-session memory extraction and consolidation." },
        FieldGroup { id: "tui", title: "TUI", description: "Notifications, animations, tooltips, alternate screen, vim." },
        FieldGroup { id: "privacy", title: "Privacy & telemetry", description: "Analytics, feedback, notify hooks." },
        FieldGroup { id: "otel", title: "OpenTelemetry", description: "OTLP export of logs/metrics." },
        FieldGroup { id: "auth", title: "Auth & updates", description: "Credential store, ChatGPT login, update checks." },
    ]
}

fn fields() -> Vec<ConfigField> {
    vec![
        // Core
        ConfigField { id: "model", path: "model", section: "", group: "core", label: "Model", description: "Primary model id (e.g. gpt-5.4, gpt-5.3-codex, gpt-5.4-mini).", field_type: "string", default: Some(jstr("gpt-5.4")), options: None, env: None, cli: Some("-m / --model"), recommended: Some(true) },
        ConfigField { id: "review_model", path: "review_model", section: "", group: "core", label: "Review model", description: "Model used for /review and codex review.", field_type: "string", default: Some(jstr("gpt-5.3-codex")), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "model_provider", path: "model_provider", section: "", group: "core", label: "Model provider", description: "Built-in or custom provider id (openai, ollama, lmstudio, amazon-bedrock).", field_type: "string", default: Some(jstr("openai")), options: None, env: None, cli: Some("-c model_provider=..."), recommended: Some(true) },
        ConfigField { id: "oss_provider", path: "oss_provider", section: "", group: "core", label: "OSS provider", description: "Default local provider when using --oss (ollama | lmstudio).", field_type: "enum", default: Some(jstr("ollama")), options: Some(&["ollama", "lmstudio"]), env: None, cli: Some("--local-provider"), recommended: None },
        ConfigField { id: "openai_base_url", path: "openai_base_url", section: "", group: "core", label: "OpenAI base URL", description: "Override built-in OpenAI provider base URL.", field_type: "string", default: None, options: None, env: Some("OPENAI_BASE_URL"), cli: None, recommended: None },
        ConfigField { id: "chatgpt_base_url", path: "chatgpt_base_url", section: "", group: "core", label: "ChatGPT base URL", description: "Override base URL used during ChatGPT login flow.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model_catalog_json", path: "model_catalog_json", section: "", group: "core", label: "Model catalog JSON", description: "Path to a custom model catalog file.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model_instructions_file", path: "model_instructions_file", section: "", group: "core", label: "Model instructions file", description: "Path to a file injected as model instructions.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "personality", path: "personality", section: "", group: "core", label: "Personality", description: "Agent tone/style when features.personality is on.", field_type: "enum", default: Some(jstr("pragmatic")), options: Some(&["pragmatic", "concise", "mentorial", "bold", "friendly", "none"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "service_tier", path: "service_tier", section: "", group: "core", label: "Service tier", description: "OpenAI service tier preference.", field_type: "enum", default: Some(jstr("flex")), options: Some(&["auto", "default", "flex", "priority", "fast"]), env: None, cli: None, recommended: None },
        ConfigField { id: "file_opener", path: "file_opener", section: "", group: "core", label: "File opener", description: "IDE used when opening files from the TUI.", field_type: "enum", default: Some(jstr("vscode")), options: Some(&["vscode", "vscode-insiders", "cursor", "windsurf", "none"]), env: None, cli: None, recommended: None },
        ConfigField { id: "developer_instructions", path: "developer_instructions", section: "", group: "core", label: "Developer instructions", description: "Always-on developer instructions (like AGENTS.md but global).", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "instructions", path: "instructions", section: "", group: "core", label: "Instructions", description: "Additional free-form instructions string.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model_context_window", path: "model_context_window", section: "", group: "core", label: "Model context window", description: "Override context window size for the model.", field_type: "number", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model_auto_compact_token_limit", path: "model_auto_compact_token_limit", section: "", group: "core", label: "Auto-compact token limit", description: "Trigger history compaction at this token count.", field_type: "number", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "compact_prompt", path: "compact_prompt", section: "", group: "core", label: "Compact prompt", description: "Inline override for the history compaction prompt.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "commit_attribution", path: "commit_attribution", section: "", group: "core", label: "Commit attribution", description: "String used when codex_git_commit makes commits.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "background_terminal_max_timeout", path: "background_terminal_max_timeout", section: "", group: "core", label: "Background terminal max timeout", description: "Max poll window ms for empty write_stdin polls (default 300000).", field_type: "number", default: Some(jnum(300000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "allow_login_shell", path: "allow_login_shell", section: "", group: "core", label: "Allow login shell", description: "Allow shell tools to use login-shell semantics.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "disable_paste_burst", path: "disable_paste_burst", section: "", group: "core", label: "Disable paste burst", description: "Disable multi-line paste burst handling.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "log_dir", path: "log_dir", section: "", group: "core", label: "Log directory", description: "Directory for Codex logs.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },

        // Reasoning
        ConfigField { id: "model_reasoning_effort", path: "model_reasoning_effort", section: "", group: "reasoning", label: "Reasoning effort", description: "How hard the model thinks before acting.", field_type: "enum", default: Some(jstr("high")), options: Some(&["minimal", "low", "medium", "high", "xhigh"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "model_reasoning_summary", path: "model_reasoning_summary", section: "", group: "reasoning", label: "Reasoning summary", description: "Summary style for reasoning traces.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "concise", "detailed", "none"]), env: None, cli: None, recommended: None },
        ConfigField { id: "model_verbosity", path: "model_verbosity", section: "", group: "reasoning", label: "Model verbosity", description: "Output verbosity preference.", field_type: "enum", default: Some(jstr("medium")), options: Some(&["low", "medium", "high"]), env: None, cli: None, recommended: None },
        ConfigField { id: "plan_mode_reasoning_effort", path: "plan_mode_reasoning_effort", section: "", group: "reasoning", label: "Plan-mode effort", description: "Reasoning effort override inside /plan.", field_type: "enum", default: Some(jstr("xhigh")), options: Some(&["minimal", "low", "medium", "high", "xhigh"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "hide_agent_reasoning", path: "hide_agent_reasoning", section: "", group: "reasoning", label: "Hide agent reasoning", description: "Suppress reasoning events in TUI / exec.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "show_raw_agent_reasoning", path: "show_raw_agent_reasoning", section: "", group: "reasoning", label: "Show raw reasoning", description: "Surface raw reasoning when provider supports it.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model_supports_reasoning_summaries", path: "model_supports_reasoning_summaries", section: "", group: "reasoning", label: "Supports reasoning summaries", description: "Advertise reasoning summary support for the model.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },

        // Sandbox
        ConfigField { id: "approval_policy", path: "approval_policy", section: "", group: "sandbox", label: "Approval policy", description: "untrusted | on-request | never (or granular table).", field_type: "enum", default: Some(jstr("on-request")), options: Some(&["untrusted", "on-request", "never"]), env: None, cli: Some("-a / --ask-for-approval"), recommended: Some(true) },
        ConfigField { id: "approval_policy.granular.sandbox_approval", path: "approval_policy.granular.sandbox_approval", section: "approval_policy.granular", group: "sandbox", label: "Granular: sandbox approval", description: "When true, sandbox escalation prompts surface.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "approval_policy.granular.rules", path: "approval_policy.granular.rules", section: "approval_policy.granular", group: "sandbox", label: "Granular: rules", description: "When true, execpolicy prompt rules surface.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "approval_policy.granular.mcp_elicitations", path: "approval_policy.granular.mcp_elicitations", section: "approval_policy.granular", group: "sandbox", label: "Granular: MCP elicitations", description: "When true, MCP elicitation prompts surface.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "approval_policy.granular.request_permissions", path: "approval_policy.granular.request_permissions", section: "approval_policy.granular", group: "sandbox", label: "Granular: request_permissions", description: "When true, request_permissions tool prompts surface.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "approval_policy.granular.skill_approval", path: "approval_policy.granular.skill_approval", section: "approval_policy.granular", group: "sandbox", label: "Granular: skill approval", description: "When true, skill-script approval prompts surface.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "sandbox_mode", path: "sandbox_mode", section: "", group: "sandbox", label: "Sandbox mode", description: "read-only | workspace-write | danger-full-access.", field_type: "enum", default: Some(jstr("workspace-write")), options: Some(&["read-only", "workspace-write", "danger-full-access"]), env: None, cli: Some("-s / --sandbox"), recommended: Some(true) },
        ConfigField { id: "sandbox_workspace_write.network_access", path: "sandbox_workspace_write.network_access", section: "sandbox_workspace_write", group: "sandbox", label: "Workspace network access", description: "Allow outbound HTTP inside workspace-write sandbox.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "sandbox_workspace_write.exclude_slash_tmp", path: "sandbox_workspace_write.exclude_slash_tmp", section: "sandbox_workspace_write", group: "sandbox", label: "Exclude /tmp", description: "Drop /tmp from writable set.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "sandbox_workspace_write.exclude_tmpdir_env_var", path: "sandbox_workspace_write.exclude_tmpdir_env_var", section: "sandbox_workspace_write", group: "sandbox", label: "Exclude $TMPDIR", description: "Drop $TMPDIR from writable set.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "sandbox_workspace_write.writable_roots", path: "sandbox_workspace_write.writable_roots", section: "sandbox_workspace_write", group: "sandbox", label: "Writable roots", description: "Extra writable roots (comma-separated paths).", field_type: "string-list", default: None, options: None, env: None, cli: Some("--add-dir"), recommended: None },
        ConfigField { id: "approvals_reviewer", path: "approvals_reviewer", section: "", group: "sandbox", label: "Approvals reviewer", description: "user | auto_review.", field_type: "enum", default: Some(jstr("user")), options: Some(&["user", "auto_review"]), env: None, cli: None, recommended: None },
        ConfigField { id: "auto_review.policy", path: "auto_review.policy", section: "auto_review", group: "sandbox", label: "Auto-review policy", description: "Local reviewer policy instructions when approvals_reviewer=auto_review.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },

        // Permissions
        ConfigField { id: "default_permissions", path: "default_permissions", section: "", group: "permissions", label: "Default permissions profile", description: "Built-ins: :read-only, :workspace, :danger-full-access.", field_type: "string", default: Some(jstr(":workspace")), options: None, env: None, cli: None, recommended: Some(true) },

        // Agents
        ConfigField { id: "agents.enabled", path: "agents.enabled", section: "agents", group: "agents", label: "Multi-agent enabled", description: "Enable or disable multi-agent tools.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "agents.default_subagent_model", path: "agents.default_subagent_model", section: "agents", group: "agents", label: "Default subagent model", description: "Default model for spawned agents.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "agents.default_subagent_reasoning_effort", path: "agents.default_subagent_reasoning_effort", section: "agents", group: "agents", label: "Default subagent effort", description: "Default reasoning effort for spawned agents.", field_type: "enum", default: Some(jstr("medium")), options: Some(&["minimal", "low", "medium", "high", "xhigh"]), env: None, cli: None, recommended: None },
        ConfigField { id: "agents.max_concurrent_threads_per_session", path: "agents.max_concurrent_threads_per_session", section: "agents", group: "agents", label: "Max concurrent agent threads", description: "Max spawned-agent threads open concurrently (excludes primary).", field_type: "number", default: Some(jnum(4.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "agents.interrupt_message", path: "agents.interrupt_message", section: "agents", group: "agents", label: "Interrupt message", description: "Record a model-visible message when an agent turn is interrupted.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },

        // Features
        ConfigField { id: "features.hooks", path: "features.hooks", section: "features", group: "features", label: "Hooks", description: "Enable lifecycle hooks.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "features.multi_agent", path: "features.multi_agent", section: "features", group: "features", label: "Multi-agent", description: "Enable multi-agent orchestration.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "features.memories", path: "features.memories", section: "features", group: "features", label: "Memories", description: "Cross-session memory feature.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.undo", path: "features.undo", section: "features", group: "features", label: "Undo", description: "Enable undo for tool side-effects when supported.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.fast_mode", path: "features.fast_mode", section: "features", group: "features", label: "Fast mode", description: "Bias toward lower latency.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.shell_tool", path: "features.shell_tool", section: "features", group: "features", label: "Shell tool", description: "Enable the shell/exec tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.unified_exec", path: "features.unified_exec", section: "features", group: "features", label: "Unified exec", description: "PTY-backed unified execution tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.shell_snapshot", path: "features.shell_snapshot", section: "features", group: "features", label: "Shell snapshot", description: "Snapshot shell environment between commands.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.personality", path: "features.personality", section: "features", group: "features", label: "Personality feature", description: "Honor the personality key.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.network_proxy", path: "features.network_proxy", section: "features", group: "features", label: "Network proxy feature", description: "Enable features.network_proxy stack.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.codex_git_commit", path: "features.codex_git_commit", section: "features", group: "features", label: "Git commit tool", description: "Enable codex git commit helper.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.web_search_request", path: "features.web_search_request", section: "features", group: "features", label: "Web search request", description: "Feature flag for live web search requests.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: Some("--search"), recommended: None },
        ConfigField { id: "features.web_search_cached", path: "features.web_search_cached", section: "features", group: "features", label: "Web search cached", description: "Feature flag for cached web search.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.web_search", path: "features.web_search", section: "features", group: "features", label: "Web search feature", description: "Master web search feature flag.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.plugins", path: "features.plugins", section: "features", group: "features", label: "Plugins", description: "Enable plugin system.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.apps", path: "features.apps", section: "features", group: "features", label: "Apps", description: "Enable Apps integration.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.goals", path: "features.goals", section: "features", group: "features", label: "Goals", description: "Enable goals tracking feature.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.prevent_idle_sleep", path: "features.prevent_idle_sleep", section: "features", group: "features", label: "Prevent idle sleep", description: "Keep machine awake during long agent runs.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.enable_request_compression", path: "features.enable_request_compression", section: "features", group: "features", label: "Request compression", description: "Enable HTTP request compression to the API.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.skill_mcp_dependency_install", path: "features.skill_mcp_dependency_install", section: "features", group: "features", label: "Skill MCP dependency install", description: "Allow skills to install MCP dependencies.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.remote_plugin", path: "features.remote_plugin", section: "features", group: "features", label: "Remote plugin", description: "Enable remote plugin loading.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.guardian_approval", path: "features.guardian_approval", section: "features", group: "features", label: "Guardian approval", description: "Enable Guardian approval flows.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },

        // History
        ConfigField { id: "history.persistence", path: "history.persistence", section: "history", group: "history", label: "History persistence", description: "save-all | none.", field_type: "enum", default: Some(jstr("save-all")), options: Some(&["save-all", "none"]), env: None, cli: None, recommended: None },
        ConfigField { id: "history.max_bytes", path: "history.max_bytes", section: "history", group: "history", label: "History max bytes", description: "Cap stored history size.", field_type: "number", default: Some(jnum(52428800.0)), options: None, env: None, cli: None, recommended: None },

        // Shell
        ConfigField { id: "shell_environment_policy.inherit", path: "shell_environment_policy.inherit", section: "shell_environment_policy", group: "shell", label: "Env inherit", description: "all | core | none.", field_type: "enum", default: Some(jstr("core")), options: Some(&["all", "core", "none"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "shell_environment_policy.ignore_default_excludes", path: "shell_environment_policy.ignore_default_excludes", section: "shell_environment_policy", group: "shell", label: "Ignore default excludes", description: "Disable automatic KEY/SECRET/TOKEN filtering.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "shell_environment_policy.exclude", path: "shell_environment_policy.exclude", section: "shell_environment_policy", group: "shell", label: "Env exclude globs", description: "Comma-separated globs to exclude (e.g. AWS_*, AZURE_*).", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "shell_environment_policy.include_only", path: "shell_environment_policy.include_only", section: "shell_environment_policy", group: "shell", label: "Env include-only", description: "If set, only these vars are passed (comma-separated).", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "shell_environment_policy.experimental_use_profile", path: "shell_environment_policy.experimental_use_profile", section: "shell_environment_policy", group: "shell", label: "Use shell profile", description: "Load shell profile on subprocess spawn.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },

        // Project
        ConfigField { id: "project_doc_max_bytes", path: "project_doc_max_bytes", section: "", group: "project", label: "Project doc max bytes", description: "Max size for project instruction docs.", field_type: "number", default: Some(jnum(65536.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "project_doc_fallback_filenames", path: "project_doc_fallback_filenames", section: "", group: "project", label: "Project doc fallbacks", description: "Fallback filenames if AGENTS.md is missing (comma-separated).", field_type: "string-list", default: Some(serde_json::json!([".cursorrules", "CLAUDE.md"])), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "project_root_markers", path: "project_root_markers", section: "", group: "project", label: "Project root markers", description: "Markers to detect project root (comma-separated).", field_type: "string-list", default: Some(serde_json::json!([".git", ".hg", ".sl"])), options: None, env: None, cli: None, recommended: None },

        // Web
        ConfigField { id: "web_search", path: "web_search", section: "", group: "web", label: "Web search mode", description: "disabled | cached | indexed | live.", field_type: "enum", default: Some(jstr("cached")), options: Some(&["disabled", "cached", "indexed", "live", "off"]), env: None, cli: Some("--search"), recommended: None },

        // Provider
        ConfigField { id: "provider.id", path: "model_providers.custom.id", section: "model_providers", group: "provider", label: "Custom provider id", description: "Table name under [model_providers.id].", field_type: "string", default: Some(jstr("openai_proxy")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.name", path: "model_providers.custom.name", section: "model_providers", group: "provider", label: "Provider display name", description: "Human label for the custom provider.", field_type: "string", default: Some(jstr("OpenAI-compatible")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.base_url", path: "model_providers.custom.base_url", section: "model_providers", group: "provider", label: "Provider base URL", description: "OpenAI-compatible base URL.", field_type: "string", default: Some(jstr("https://api.openai.com/v1")), options: None, env: Some("OPENAI_BASE_URL"), cli: None, recommended: None },
        ConfigField { id: "provider.env_key", path: "model_providers.custom.env_key", section: "model_providers", group: "provider", label: "API key env var", description: "Env var holding the API key.", field_type: "string", default: Some(jstr("OPENAI_API_KEY")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.env_key_instructions", path: "model_providers.custom.env_key_instructions", section: "model_providers", group: "provider", label: "Env key instructions", description: "Help text for obtaining the API key.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.wire_api", path: "model_providers.custom.wire_api", section: "model_providers", group: "provider", label: "Wire API", description: "API dialect (responses recommended).", field_type: "enum", default: Some(jstr("responses")), options: Some(&["responses", "chat"]), env: None, cli: None, recommended: None },
        ConfigField { id: "provider.requires_openai_auth", path: "model_providers.custom.requires_openai_auth", section: "model_providers", group: "provider", label: "Requires OpenAI auth", description: "Whether auth must mimic OpenAI ChatGPT login.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.request_max_retries", path: "model_providers.custom.request_max_retries", section: "model_providers", group: "provider", label: "Request max retries", description: "HTTP request retry count for the provider.", field_type: "number", default: Some(jnum(4.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.stream_max_retries", path: "model_providers.custom.stream_max_retries", section: "model_providers", group: "provider", label: "Stream max retries", description: "Streaming retry count.", field_type: "number", default: Some(jnum(10.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.stream_idle_timeout_ms", path: "model_providers.custom.stream_idle_timeout_ms", section: "model_providers", group: "provider", label: "Stream idle timeout ms", description: "Abort stream if idle longer than this.", field_type: "number", default: Some(jnum(300000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.supports_websockets", path: "model_providers.custom.supports_websockets", section: "model_providers", group: "provider", label: "Supports websockets", description: "Whether this provider supports websocket transport.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },

        // MCP sample
        ConfigField { id: "mcp.id", path: "mcp_servers.sample.id", section: "mcp_servers", group: "mcp", label: "MCP server id", description: "Table name under [mcp_servers.id].", field_type: "string", default: Some(jstr("github")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.enabled", path: "mcp_servers.sample.enabled", section: "mcp_servers", group: "mcp", label: "MCP enabled", description: "Enable this MCP server entry.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.command", path: "mcp_servers.sample.command", section: "mcp_servers", group: "mcp", label: "MCP command", description: "Stdio command argv as comma-separated (e.g. npx,-y,@modelcontextprotocol/server-github).", field_type: "string-list", default: Some(serde_json::json!(["npx", "-y", "@modelcontextprotocol/server-github"])), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.url", path: "mcp_servers.sample.url", section: "mcp_servers", group: "mcp", label: "MCP remote URL", description: "Remote MCP server URL (alternative to command).", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.startup_timeout_sec", path: "mcp_servers.sample.startup_timeout_sec", section: "mcp_servers", group: "mcp", label: "MCP startup timeout", description: "Seconds to wait for MCP server startup.", field_type: "number", default: Some(jnum(30.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.tool_timeout_sec", path: "mcp_servers.sample.tool_timeout_sec", section: "mcp_servers", group: "mcp", label: "MCP tool timeout", description: "Per-tool timeout in seconds.", field_type: "number", default: Some(jnum(60.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.bearer_token_env_var", path: "mcp_servers.sample.bearer_token_env_var", section: "mcp_servers", group: "mcp", label: "MCP bearer token env", description: "Env var holding bearer token for remote MCP.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp_oauth_credentials_store", path: "mcp_oauth_credentials_store", section: "", group: "mcp", label: "MCP OAuth credentials store", description: "Where to store MCP OAuth credentials (file | keyring | auto).", field_type: "enum", default: Some(jstr("auto")), options: Some(&["file", "keyring", "auto"]), env: None, cli: None, recommended: None },

        // Memories
        ConfigField { id: "memories.use_memories", path: "memories.use_memories", section: "memories", group: "memories", label: "Use memories", description: "Inject memories into sessions.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "memories.generate_memories", path: "memories.generate_memories", section: "memories", group: "memories", label: "Generate memories", description: "Extract memories from rollouts.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "memories.extract_model", path: "memories.extract_model", section: "memories", group: "memories", label: "Extract model", description: "Model used for memory extraction.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "memories.consolidation_model", path: "memories.consolidation_model", section: "memories", group: "memories", label: "Consolidation model", description: "Model used for memory consolidation.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "memories.max_rollout_age_days", path: "memories.max_rollout_age_days", section: "memories", group: "memories", label: "Max rollout age days", description: "Ignore rollouts older than this many days.", field_type: "number", default: Some(jnum(30.0)), options: None, env: None, cli: None, recommended: None },

        // TUI
        ConfigField { id: "tui.notifications", path: "tui.notifications", section: "tui", group: "tui", label: "TUI notifications", description: "true | false — desktop/TUI notifications.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tui.notification_method", path: "tui.notification_method", section: "tui", group: "tui", label: "Notification method", description: "auto | osc9 | bel.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "osc9", "bel"]), env: None, cli: None, recommended: None },
        ConfigField { id: "tui.notification_condition", path: "tui.notification_condition", section: "tui", group: "tui", label: "Notification condition", description: "unfocused | always.", field_type: "enum", default: Some(jstr("unfocused")), options: Some(&["unfocused", "always"]), env: None, cli: None, recommended: None },
        ConfigField { id: "tui.animations", path: "tui.animations", section: "tui", group: "tui", label: "Animations", description: "ASCII animations and shimmer.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tui.alternate_screen", path: "tui.alternate_screen", section: "tui", group: "tui", label: "Alternate screen", description: "auto | always | never — set never to keep scrollback.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "always", "never"]), env: None, cli: None, recommended: None },
        ConfigField { id: "tui.show_tooltips", path: "tui.show_tooltips", section: "tui", group: "tui", label: "Show tooltips", description: "Onboarding tooltips on welcome screen.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tui.theme", path: "tui.theme", section: "tui", group: "tui", label: "TUI theme", description: "Theme name (kebab-case).", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tui.vim_mode_default", path: "tui.vim_mode_default", section: "tui", group: "tui", label: "Vim mode default", description: "Start TUI in Vim mode.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tui.raw_output_mode", path: "tui.raw_output_mode", section: "tui", group: "tui", label: "Raw output mode", description: "Show raw model/tool output.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },

        // Privacy
        ConfigField { id: "analytics.enabled", path: "analytics.enabled", section: "analytics", group: "privacy", label: "Analytics", description: "Anonymous usage metrics.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "feedback.enabled", path: "feedback.enabled", section: "feedback", group: "privacy", label: "Feedback", description: "Enable /feedback submission.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "notify", path: "notify", section: "", group: "privacy", label: "Notify command", description: "Command argv for event notifications (comma-separated).", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },

        // OTel
        ConfigField { id: "otel.environment", path: "otel.environment", section: "otel", group: "otel", label: "OTel environment", description: "dev | staging | prod label.", field_type: "enum", default: Some(jstr("dev")), options: Some(&["dev", "staging", "prod"]), env: None, cli: None, recommended: None },
        ConfigField { id: "otel.exporter", path: "otel.exporter", section: "otel", group: "otel", label: "OTel exporter", description: "none | otlp-http | otlp-grpc.", field_type: "enum", default: Some(jstr("none")), options: Some(&["none", "otlp-http", "otlp-grpc"]), env: None, cli: None, recommended: None },
        ConfigField { id: "otel.log_user_prompt", path: "otel.log_user_prompt", section: "otel", group: "otel", label: "Log user prompts", description: "Include user prompts in OTel logs (privacy-sensitive).", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },

        // Auth & updates
        ConfigField { id: "check_for_update_on_startup", path: "check_for_update_on_startup", section: "", group: "auth", label: "Check for update on startup", description: "Check for Codex updates when starting.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "cli_auth_credentials_store", path: "cli_auth_credentials_store", section: "", group: "auth", label: "CLI auth credentials store", description: "file | keyring | auto.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["file", "keyring", "auto"]), env: None, cli: None, recommended: None },
        ConfigField { id: "forced_login_method", path: "forced_login_method", section: "", group: "auth", label: "Forced login method", description: "Force a specific login method when set.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
    ]
}

fn cli_flags() -> &'static [CliFlag] {
    &[
        CliFlag { flag: "codex", description: "Start interactive TUI", category: "Session" },
        CliFlag { flag: "codex exec <PROMPT>", description: "Headless one-shot", category: "Session" },
        CliFlag { flag: "codex review", description: "Non-interactive code review", category: "Session" },
        CliFlag { flag: "codex resume", description: "Resume a previous session", category: "Session" },
        CliFlag { flag: "codex fork", description: "Fork a previous session", category: "Session" },
        CliFlag { flag: "-m / --model <MODEL>", description: "Model override", category: "Model" },
        CliFlag { flag: "-i / --image <FILE>", description: "Attach image to initial prompt", category: "Session" },
        CliFlag { flag: "-C / --cd <DIR>", description: "Working directory", category: "Session" },
        CliFlag { flag: "--add-dir <DIR>", description: "Extra writable directory", category: "Permissions" },
        CliFlag { flag: "-p / --profile <NAME>", description: "Apply profile overlay", category: "Config" },
        CliFlag { flag: "-c / --config KEY=VALUE", description: "Override any config key (TOML value)", category: "Config" },
        CliFlag { flag: "-s / --sandbox <MODE>", description: "Sandbox override", category: "Permissions" },
        CliFlag { flag: "-a / --ask-for-approval <MODE>", description: "Approval policy override", category: "Permissions" },
        CliFlag { flag: "--full-auto", description: "on-request + workspace-write autonomous mode", category: "Permissions" },
        CliFlag { flag: "--yolo", description: "No sandbox, never approve (dangerous)", category: "Permissions" },
        CliFlag { flag: "--search", description: "Enable live web search", category: "Tools" },
        CliFlag { flag: "--oss", description: "Use local OSS model (Ollama/LM Studio)", category: "Model" },
        CliFlag { flag: "--local-provider <NAME>", description: "ollama | lmstudio", category: "Model" },
        CliFlag { flag: "--json", description: "JSON event stream (exec)", category: "Headless" },
        CliFlag { flag: "-o / --output <FILE>", description: "Write final response to file", category: "Headless" },
        CliFlag { flag: "--output-schema <FILE>", description: "Enforce JSON schema on output", category: "Headless" },
        CliFlag { flag: "--version", description: "Show version", category: "Global" },
    ]
}

fn env_vars() -> &'static [EnvVar] {
    &[
        EnvVar { name: "CODEX_HOME", description: "Config/state home (default ~/.codex)", category: "Paths" },
        EnvVar { name: "CODEX_API_KEY", description: "API key for non-interactive/CI use", category: "Auth" },
        EnvVar { name: "OPENAI_API_KEY", description: "API key for OpenAI provider", category: "Auth" },
        EnvVar { name: "OPENAI_BASE_URL", description: "Override OpenAI-compatible base URL", category: "Auth" },
        EnvVar { name: "CODEX_CA_CERTIFICATE", description: "Custom CA cert for corporate proxies", category: "Network" },
        EnvVar { name: "CODEX_THREAD_ID", description: "Current thread ID injected into subprocesses", category: "Runtime" },
        EnvVar { name: "HTTPS_PROXY", description: "HTTP proxy for corporate networks", category: "Network" },
        EnvVar { name: "HTTP_PROXY", description: "HTTP proxy", category: "Network" },
        EnvVar { name: "NO_PROXY", description: "Proxy bypass", category: "Network" },
        EnvVar { name: "OTLP_TOKEN", description: "Token for OTel exporter headers", category: "Telemetry" },
        EnvVar { name: "RUST_LOG", description: "Log filter", category: "Logging" },
    ]
}

fn subcommands() -> &'static [Subcommand] {
    &[
        Subcommand { cmd: "codex", desc: "Interactive coding agent TUI" },
        Subcommand { cmd: "codex exec", desc: "Non-interactive prompt execution" },
        Subcommand { cmd: "codex review", desc: "Non-interactive code review" },
        Subcommand { cmd: "codex resume", desc: "Resume a previous session" },
        Subcommand { cmd: "codex fork", desc: "Fork a previous session into a new thread" },
        Subcommand { cmd: "codex apply", desc: "Apply latest diff as git apply" },
        Subcommand { cmd: "codex login", desc: "Authenticate" },
        Subcommand { cmd: "codex logout", desc: "Clear credentials" },
        Subcommand { cmd: "codex mcp", desc: "Manage MCP servers" },
        Subcommand { cmd: "codex mcp-server", desc: "Run Codex as an MCP server (stdio)" },
        Subcommand { cmd: "codex sandbox", desc: "Run commands within Codex sandbox" },
        Subcommand { cmd: "codex features list", desc: "List feature flags" },
        Subcommand { cmd: "codex cloud", desc: "Browse and apply Codex Cloud tasks" },
        Subcommand { cmd: "codex app", desc: "Launch desktop app (macOS)" },
        Subcommand { cmd: "codex completion", desc: "Shell completions" },
        Subcommand { cmd: "codex execpolicy check", desc: "Test execution policy rules" },
    ]
}

fn presets() -> Vec<Preset> {
    vec![
        Preset {
            id: "titanium",
            name: "Titanium (default)",
            description: "Sensible high-power defaults: gpt-5.4, high effort, workspace-write + network, multi-agent, hooks, memories.",
            enabled: &[
                "model", "review_model", "model_provider", "personality", "service_tier", "file_opener",
                "model_reasoning_effort", "model_reasoning_summary", "model_verbosity", "plan_mode_reasoning_effort",
                "approval_policy", "sandbox_mode", "sandbox_workspace_write.network_access", "default_permissions",
                "agents.enabled", "agents.max_concurrent_threads_per_session",
                "features.hooks", "features.multi_agent", "features.memories", "features.undo",
                "features.shell_tool", "features.personality", "features.codex_git_commit", "features.web_search_request",
                "features.unified_exec", "features.apps", "features.goals",
                "history.persistence", "shell_environment_policy.inherit", "web_search",
                "tui.notifications", "analytics.enabled", "check_for_update_on_startup",
            ],
            values: serde_json::json!({
                "model": "gpt-5.4",
                "review_model": "gpt-5.3-codex",
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
                "agents.enabled": true,
                "agents.max_concurrent_threads_per_session": 4,
                "features.hooks": true,
                "features.multi_agent": true,
                "features.memories": true,
                "features.undo": true,
                "features.shell_tool": true,
                "features.personality": true,
                "features.codex_git_commit": true,
                "features.web_search_request": true,
                "features.unified_exec": true,
                "features.apps": true,
                "features.goals": true,
                "history.persistence": "save-all",
                "shell_environment_policy.inherit": "core",
                "web_search": "cached",
                "tui.notifications": true,
                "analytics.enabled": true,
                "check_for_update_on_startup": true
            }),
        },
        Preset {
            id: "titanium-yolo",
            name: "Titanium YOLO",
            description: "Still titanium models, but never-approve + danger-full-access. Throwaway sandboxes only.",
            enabled: &["model", "model_provider", "model_reasoning_effort", "approval_policy", "sandbox_mode", "default_permissions", "features.multi_agent", "features.hooks", "agents.enabled"],
            values: serde_json::json!({
                "model": "gpt-5.4",
                "model_provider": "openai",
                "model_reasoning_effort": "high",
                "approval_policy": "never",
                "sandbox_mode": "danger-full-access",
                "default_permissions": ":danger-full-access",
                "features.multi_agent": true,
                "features.hooks": true,
                "agents.enabled": true
            }),
        },
        Preset {
            id: "read-only-review",
            name: "Read-only review",
            description: "Safe PR review: read-only sandbox, untrusted approvals.",
            enabled: &["model", "review_model", "sandbox_mode", "approval_policy", "default_permissions", "model_reasoning_effort", "features.shell_tool"],
            values: serde_json::json!({
                "model": "gpt-5.4",
                "review_model": "gpt-5.3-codex",
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
            enabled: &["model", "model_provider", "oss_provider", "provider.id", "provider.name", "provider.base_url", "provider.env_key", "sandbox_mode", "approval_policy"],
            values: serde_json::json!({
                "model": "gpt-oss:120b",
                "model_provider": "ollama",
                "oss_provider": "ollama",
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
        "string-list" => {
            let list: Vec<String> = if let Some(arr) = v.as_array() {
                arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect()
            } else if let Some(s) = v.as_str() {
                s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()
            } else {
                return None;
            };
            if list.is_empty() { return None; }
            Some(format!("[{}]", list.iter().map(|s| esc(s)).collect::<Vec<_>>().join(", ")))
        }
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
    let mut mcp_id = "github".to_string();
    if let Some(Value::String(s)) = values.get("mcp.id") { mcp_id = s.clone(); }

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
        if f.id.starts_with("mcp.") {
            if f.id == "mcp.id" { continue; }
            let key = match f.id {
                "mcp.enabled" => "enabled",
                "mcp.command" => "command",
                "mcp.url" => "url",
                "mcp.startup_timeout_sec" => "startup_timeout_sec",
                "mcp.tool_timeout_sec" => "tool_timeout_sec",
                "mcp.bearer_token_env_var" => "bearer_token_env_var",
                _ => continue,
            };
            sections.entry(format!("mcp_servers.{}", mcp_id)).or_default().push(format!("{} = {}", key, formatted));
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
    let order = [
        "features", "agents", "sandbox_workspace_write", "approval_policy.granular", "history",
        "shell_environment_policy", "tui", "analytics", "feedback", "otel", "auto_review", "memories",
    ];
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
    env.push_str("# export OPENAI_API_KEY=sk-...\n# export CODEX_HOME=~/.codex\n# export CODEX_API_KEY=...\n");

    let mut cli = format!("# Codex Titanium launch snippets\n# Generated: {}\n\n", now);
    cli.push_str("codex --profile titanium\n");
    if let Some(Value::String(m)) = values.get("model") {
        if en.contains("model") { cli.push_str(&format!("codex -c model={}\n", esc(m))); }
    }
    cli.push_str("codex exec \"review this PR for security issues\"\n");
    cli.push_str("codex review\n");
    cli.push_str("codex --full-auto\n");

    let md = markdown();
    GenerateOut { config: out.clone(), toml: out, env, cli, markdown: md }
}

fn markdown() -> String {
    let mut md = String::from("# Codex Titanium — Config Reference\n\n");
    md.push_str("> Opinionated **Titanium** defaults for OpenAI Codex CLI (`~/.codex/config.toml`).\n\n");
    md.push_str("## Install\n\n```bash\nnpm i -g @openai/codex\ncodex --version\n```\n\n");
    md.push_str("## Titanium default philosophy\n\n- Model: `gpt-5.4` with **high** reasoning (xhigh in plan mode)\n- Sandbox: `workspace-write` + network\n- Approvals: `on-request`\n- Features: multi-agent, hooks, memories, undo, apps, goals\n- Activate: `codex --profile titanium`\n\n");
    md.push_str("## Subcommands\n\n| Command | Description |\n| --- | --- |\n");
    for s in subcommands() { md.push_str(&format!("| `{}` | {} |\n", s.cmd, s.desc)); }
    md.push_str("\n## Flags\n\n| Flag | Category | Description |\n| --- | --- | --- |\n");
    for f in cli_flags() { md.push_str(&format!("| `{}` | {} | {} |\n", f.flag, f.category, f.description)); }
    md.push_str("\n## Env\n\n| Var | Category | Description |\n| --- | --- | --- |\n");
    for e in env_vars() { md.push_str(&format!("| `{}` | {} | {} |\n", e.name, e.category, e.description)); }
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
