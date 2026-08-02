//! Grok Build CLI config schema — single source of truth (Rust).
//! Exhaustive knobs from docs.x.ai/build/settings/reference + CLI reference.

use serde::Serialize;

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
    pub default: Option<serde_json::Value>,
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
    pub values: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaPayload {
    pub groups: &'static [FieldGroup],
    pub fields: Vec<ConfigField>,
    pub cli_flags: &'static [CliFlag],
    pub env_vars: &'static [EnvVar],
    pub subcommands: &'static [Subcommand],
    pub presets: Vec<Preset>,
    pub version_note: &'static str,
}

fn jstr(s: &str) -> serde_json::Value {
    serde_json::Value::String(s.to_string())
}
fn jnum(n: f64) -> serde_json::Value {
    serde_json::json!(n)
}
fn jbool(b: bool) -> serde_json::Value {
    serde_json::Value::Bool(b)
}

pub fn groups() -> &'static [FieldGroup] {
    &[
        FieldGroup { id: "models", title: "Models", description: "Default model, web search model, sampling, and catalog filters." },
        FieldGroup { id: "model-provider", title: "Custom model provider", description: "BYOK / custom endpoint for a named model (optional)." },
        FieldGroup { id: "sandbox", title: "Sandbox", description: "Filesystem sandbox profile and bash auto-allow." },
        FieldGroup { id: "permissions", title: "Permissions", description: "Permission mode and tool approval defaults." },
        FieldGroup { id: "session", title: "Session", description: "Auto-compact and session behavior." },
        FieldGroup { id: "cli", title: "CLI", description: "Auto-update channel and startup tips." },
        FieldGroup { id: "hints", title: "Worktree hints", description: "When /new and /fork offer git worktrees." },
        FieldGroup { id: "ui", title: "UI", description: "Theme, layout, thinking blocks, scroll, voice, hints." },
        FieldGroup { id: "features", title: "Features", description: "Memory, subagents, web tools, write tool, LSP, media tools." },
        FieldGroup { id: "memory", title: "Memory", description: "Cross-session memory master switch." },
        FieldGroup { id: "subagents", title: "Subagents", description: "Task/subagent master switch." },
        FieldGroup { id: "skills", title: "Skills", description: "Extra skill directories and disable list." },
        FieldGroup { id: "privacy", title: "Privacy & telemetry", description: "Codebase upload, trace upload, telemetry toggles." },
        FieldGroup { id: "auth", title: "Auth & enterprise", description: "OIDC, API key policy, team login pin." },
        FieldGroup { id: "plugins", title: "Plugins", description: "Extra plugin paths and enable/disable lists." },
        FieldGroup { id: "compat", title: "Compat (Cursor / Claude)", description: "Scan Cursor and Claude Code config surfaces." },
        FieldGroup { id: "mcp", title: "MCP sample", description: "Optional [mcp_servers.*] stdio entry." },
        FieldGroup { id: "endpoints", title: "Endpoints", description: "Custom models base URL / list URL." },
    ]
}

pub fn fields() -> Vec<ConfigField> {
    vec![
        // Models
        ConfigField { id: "models.default", path: "models.default", section: "models", group: "models", label: "Default model", description: "Model used for new sessions. Recommended: grok-build.", field_type: "string", default: Some(jstr("grok-build")), options: None, env: Some("GROK_DEFAULT_MODEL"), cli: Some("-m, --model"), recommended: Some(true) },
        ConfigField { id: "models.web_search", path: "models.web_search", section: "models", group: "models", label: "Web search model", description: "Model used by the client-side web_search tool.", field_type: "string", default: Some(jstr("grok-4.5")), options: None, env: Some("GROK_WEB_SEARCH_MODEL"), cli: None, recommended: None },
        ConfigField { id: "models.default_reasoning_effort", path: "models.default_reasoning_effort", section: "models", group: "models", label: "Default reasoning effort", description: "Default reasoning effort for the default model (if supported).", field_type: "enum", default: Some(jstr("medium")), options: Some(&["low", "medium", "high"]), env: None, cli: Some("--effort"), recommended: None },
        ConfigField { id: "models.session_summary", path: "models.session_summary", section: "models", group: "models", label: "Session summary model", description: "Model used for session titles/summaries.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.image_description", path: "models.image_description", section: "models", group: "models", label: "Image description model", description: "Model used for image description.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.temperature", path: "models.temperature", section: "models", group: "models", label: "Temperature", description: "Global sampling temperature default.", field_type: "number", default: Some(jnum(0.7)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.top_p", path: "models.top_p", section: "models", group: "models", label: "Top P", description: "Global top_p sampling default.", field_type: "number", default: Some(jnum(0.95)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.max_completion_tokens", path: "models.max_completion_tokens", section: "models", group: "models", label: "Max completion tokens", description: "Global max completion tokens.", field_type: "number", default: Some(jnum(8192.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.max_retries", path: "models.max_retries", section: "models", group: "models", label: "Max retries", description: "Global inference retry default.", field_type: "number", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.stream_tool_calls", path: "models.stream_tool_calls", section: "models", group: "models", label: "Stream tool calls", description: "Tool-call streaming request shape (some BYOK endpoints need false).", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.allowed_models", path: "models.allowed_models", section: "models", group: "models", label: "Allowed models", description: "Glob list restricting model picker / default / -m selection.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.hidden_models", path: "models.hidden_models", section: "models", group: "models", label: "Hidden models", description: "Hide from picker (still usable via -m).", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "models.disabled_models", path: "models.disabled_models", section: "models", group: "models", label: "Disabled models", description: "Remove from catalog (wins over hidden).", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },

        // Custom model
        ConfigField { id: "model.custom.id", path: "model.custom.id", section: "model.<id>", group: "model-provider", label: "Custom model section id", description: "TOML table name under [model.\"id\"]. e.g. grok-4.5", field_type: "string", default: Some(jstr("grok-4.5")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.model", path: "model.custom.model", section: "model.<id>", group: "model-provider", label: "API model id", description: "Model id sent to the API.", field_type: "string", default: Some(jstr("grok-4.5")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.base_url", path: "model.custom.base_url", section: "model.<id>", group: "model-provider", label: "Base URL", description: "Provider endpoint.", field_type: "string", default: Some(jstr("https://api.x.ai/v1")), options: None, env: Some("GROK_XAI_API_BASE_URL"), cli: None, recommended: None },
        ConfigField { id: "model.custom.name", path: "model.custom.name", section: "model.<id>", group: "model-provider", label: "Display name", description: "Label in the model picker.", field_type: "string", default: Some(jstr("Grok 4.5")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.description", path: "model.custom.description", section: "model.<id>", group: "model-provider", label: "Description", description: "Optional description in the picker.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.env_key", path: "model.custom.env_key", section: "model.<id>", group: "model-provider", label: "API key env var", description: "Environment variable holding the API key (prefer over inline key).", field_type: "string", default: Some(jstr("XAI_API_KEY")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.api_backend", path: "model.custom.api_backend", section: "model.<id>", group: "model-provider", label: "API backend", description: "Protocol: chat_completions | responses | messages.", field_type: "enum", default: Some(jstr("responses")), options: Some(&["chat_completions", "responses", "messages"]), env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.context_window", path: "model.custom.context_window", section: "model.<id>", group: "model-provider", label: "Context window", description: "Token context window (drives auto-compact timing).", field_type: "number", default: Some(jnum(1_000_000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.supports_backend_search", path: "model.custom.supports_backend_search", section: "model.<id>", group: "model-provider", label: "Supports backend search", description: "Whether endpoint supports Grok-hosted server-side search tools.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.supports_reasoning_effort", path: "model.custom.supports_reasoning_effort", section: "model.<id>", group: "model-provider", label: "Supports reasoning effort", description: "Expose reasoning effort controls for this model.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.temperature", path: "model.custom.temperature", section: "model.<id>", group: "model-provider", label: "Model temperature", description: "Per-model sampling temperature.", field_type: "number", default: Some(jnum(0.7)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.top_p", path: "model.custom.top_p", section: "model.<id>", group: "model-provider", label: "Model top_p", description: "Per-model top_p.", field_type: "number", default: Some(jnum(0.95)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.max_completion_tokens", path: "model.custom.max_completion_tokens", section: "model.<id>", group: "model-provider", label: "Model max tokens", description: "Per-model max completion tokens.", field_type: "number", default: Some(jnum(8192.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "model.custom.inference_idle_timeout_secs", path: "model.custom.inference_idle_timeout_secs", section: "model.<id>", group: "model-provider", label: "Inference idle timeout secs", description: "Abort idle inference after this many seconds.", field_type: "number", default: None, options: None, env: None, cli: None, recommended: None },

        // Endpoints
        ConfigField { id: "endpoints.models_base_url", path: "endpoints.models_base_url", section: "endpoints", group: "endpoints", label: "Models base URL", description: "Custom inference base; model list from {base}/models.", field_type: "string", default: None, options: None, env: Some("GROK_MODELS_BASE_URL"), cli: None, recommended: None },
        ConfigField { id: "endpoints.models_list_url", path: "endpoints.models_list_url", section: "endpoints", group: "endpoints", label: "Models list URL", description: "Override model-list URL when it differs from default.", field_type: "string", default: None, options: None, env: Some("GROK_MODELS_LIST_URL"), cli: None, recommended: None },

        // Sandbox
        ConfigField { id: "sandbox.profile", path: "sandbox.profile", section: "sandbox", group: "sandbox", label: "Sandbox profile", description: "off | workspace | read-only | strict | custom. Restricts FS/network for child processes.", field_type: "enum", default: Some(jstr("off")), options: Some(&["off", "workspace", "devbox", "read-only", "strict"]), env: Some("GROK_SANDBOX"), cli: Some("--sandbox"), recommended: Some(true) },
        ConfigField { id: "sandbox.auto_allow_bash", path: "sandbox.auto_allow_bash", section: "sandbox", group: "sandbox", label: "Auto-allow bash in sandbox", description: "Skip bash permission prompts when a sandbox profile is active.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_SANDBOX_AUTO_ALLOW_BASH"), cli: None, recommended: None },

        // Permissions
        ConfigField { id: "permissions.permission_mode", path: "permissions.permission_mode", section: "permissions", group: "permissions", label: "Permission mode", description: "auto | ask | always-approve. Legacy: approval_mode, yolo=true.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "ask", "always-approve"]), env: None, cli: Some("--always-approve / --yolo"), recommended: None },
        ConfigField { id: "permissions.remember_tool_approvals", path: "permissions.remember_tool_approvals", section: "permissions", group: "permissions", label: "Remember tool approvals", description: "Show per-tool Always allow options.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_REMEMBER_TOOL_APPROVALS"), cli: None, recommended: None },
        ConfigField { id: "permissions.default_selected_permission", path: "permissions.default_selected_permission", section: "permissions", group: "permissions", label: "Default selected permission", description: "Preselected row on the first permission prompt.", field_type: "enum", default: Some(jstr("always_allow_all_sessions")), options: Some(&["always_allow_all_sessions", "allow_once", "deny", "always_allow_tool", "allow_command_always", "reject"]), env: Some("GROK_DEFAULT_SELECTED_PERMISSION"), cli: None, recommended: None },

        // Session / CLI / hints
        ConfigField { id: "session.auto_compact_threshold_percent", path: "session.auto_compact_threshold_percent", section: "session", group: "session", label: "Auto-compact threshold %", description: "Auto-compact when context usage reaches this percent (0–100).", field_type: "number", default: Some(jnum(85.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "cli.auto_update", path: "cli.auto_update", section: "cli", group: "cli", label: "Auto update", description: "Check for CLI updates on launch.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_DISABLE_AUTOUPDATER"), cli: Some("--no-auto-update"), recommended: None },
        ConfigField { id: "cli.channel", path: "cli.channel", section: "cli", group: "cli", label: "Release channel", description: "stable or alpha release channel preference.", field_type: "enum", default: Some(jstr("stable")), options: Some(&["stable", "alpha"]), env: None, cli: None, recommended: None },
        ConfigField { id: "cli.show_tips", path: "cli.show_tips", section: "cli", group: "cli", label: "Show tips", description: "Show startup tips.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "hints.new_session_worktree_mode", path: "hints.new_session_worktree_mode", section: "hints", group: "hints", label: "/new worktree mode", description: "Whether /new offers a worktree: ask | always | never.", field_type: "enum", default: Some(jstr("never")), options: Some(&["ask", "always", "never"]), env: None, cli: None, recommended: None },
        ConfigField { id: "hints.fork_worktree_mode", path: "hints.fork_worktree_mode", section: "hints", group: "hints", label: "/fork worktree mode", description: "Whether /fork offers a worktree: ask | always | never.", field_type: "enum", default: Some(jstr("ask")), options: Some(&["ask", "always", "never"]), env: None, cli: None, recommended: None },

        // UI (exhaustive from settings reference)
        ConfigField { id: "ui.compact_mode", path: "ui.compact_mode", section: "ui", group: "ui", label: "Compact mode", description: "Denser message padding.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.theme", path: "ui.theme", section: "ui", group: "ui", label: "Theme", description: "Color theme (auto, system, or built-in name).", field_type: "string", default: Some(jstr("auto")), options: None, env: Some("GROK_THEME"), cli: None, recommended: None },
        ConfigField { id: "ui.auto_dark_theme", path: "ui.auto_dark_theme", section: "ui", group: "ui", label: "Auto dark theme", description: "Theme when theme=auto and OS is dark.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.auto_light_theme", path: "ui.auto_light_theme", section: "ui", group: "ui", label: "Auto light theme", description: "Theme when theme=auto and OS is light.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.screen_mode", path: "ui.screen_mode", section: "ui", group: "ui", label: "Screen mode", description: "Default render mode for plain grok: fullscreen | minimal.", field_type: "enum", default: Some(jstr("fullscreen")), options: Some(&["fullscreen", "minimal"]), env: None, cli: Some("--no-alt-screen"), recommended: None },
        ConfigField { id: "ui.show_thinking_blocks", path: "ui.show_thinking_blocks", section: "ui", group: "ui", label: "Show thinking blocks", description: "Show thinking/reasoning blocks in the TUI.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_SHOW_THINKING_BLOCKS"), cli: None, recommended: None },
        ConfigField { id: "ui.show_timestamps", path: "ui.show_timestamps", section: "ui", group: "ui", label: "Show timestamps", description: "Clock time next to messages.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.show_timeline", path: "ui.show_timeline", section: "ui", group: "ui", label: "Show timeline", description: "Per-turn tick rail instead of the scrollbar.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.page_flip_on_send", path: "ui.page_flip_on_send", section: "ui", group: "ui", label: "Page flip on send", description: "Snap the sent prompt to the top of the viewport.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.max_thoughts_width", path: "ui.max_thoughts_width", section: "ui", group: "ui", label: "Max thoughts width", description: "Column width for the thoughts panel (40–500).", field_type: "number", default: Some(jnum(80.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.combine_queued_prompts", path: "ui.combine_queued_prompts", section: "ui", group: "ui", label: "Combine queued prompts", description: "Merge consecutive plain follow-ups into one turn.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.simple_mode", path: "ui.simple_mode", section: "ui", group: "ui", label: "Simple mode", description: "Readline prompt editing (true) or experimental vim keys (false).", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.vim_mode", path: "ui.vim_mode", section: "ui", group: "ui", label: "Vim mode", description: "Vim keys in scrollback (not the prompt).", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.group_tool_verbs", path: "ui.group_tool_verbs", section: "ui", group: "ui", label: "Group tool verbs", description: "Fold consecutive read/search/list tool rows.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_GROUP_TOOL_VERBS"), cli: None, recommended: None },
        ConfigField { id: "ui.collapsed_edit_blocks", path: "ui.collapsed_edit_blocks", section: "ui", group: "ui", label: "Collapsed edit blocks", description: "Collapse edits to one-line +N/-M summaries.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_COLLAPSED_EDIT_BLOCKS"), cli: None, recommended: None },
        ConfigField { id: "ui.prompt_suggestions", path: "ui.prompt_suggestions", section: "ui", group: "ui", label: "Prompt suggestions", description: "Show next-prompt ghost text after each turn.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_PROMPT_SUGGESTIONS"), cli: None, recommended: None },
        ConfigField { id: "ui.scroll_speed", path: "ui.scroll_speed", section: "ui", group: "ui", label: "Scroll speed", description: "Mouse/trackpad scroll speed (1–100).", field_type: "number", default: Some(jnum(50.0)), options: None, env: Some("GROK_SCROLL_SPEED"), cli: None, recommended: None },
        ConfigField { id: "ui.scroll_mode", path: "ui.scroll_mode", section: "ui", group: "ui", label: "Scroll mode", description: "auto | wheel | trackpad.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "wheel", "trackpad"]), env: Some("GROK_SCROLL_MODE"), cli: None, recommended: None },
        ConfigField { id: "ui.scroll_lines", path: "ui.scroll_lines", section: "ui", group: "ui", label: "Scroll lines", description: "Lines per scroll tick (1–10).", field_type: "number", default: Some(jnum(3.0)), options: None, env: Some("GROK_SCROLL_LINES"), cli: None, recommended: None },
        ConfigField { id: "ui.invert_scroll", path: "ui.invert_scroll", section: "ui", group: "ui", label: "Invert scroll", description: "Reverse vertical scroll direction.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_INVERT_SCROLL"), cli: None, recommended: None },
        ConfigField { id: "ui.disable_bypass_permissions_mode", path: "ui.disable_bypass_permissions_mode", section: "ui", group: "ui", label: "Disable bypass-permissions mode", description: "Enterprise pin: blocks --yolo / always-approve.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.mouse_reporting_toggle", path: "ui.mouse_reporting_toggle", section: "ui", group: "ui", label: "Mouse reporting toggle", description: "Ctrl+R in scrollback toggles terminal mouse capture.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_MOUSE_REPORTING_TOGGLE"), cli: None, recommended: None },
        ConfigField { id: "ui.keep_text_selection", path: "ui.keep_text_selection", section: "ui", group: "ui", label: "Keep text selection", description: "In-app selection mode: flash | hold | word_select.", field_type: "enum", default: Some(jstr("flash")), options: Some(&["flash", "hold", "word_select"]), env: None, cli: None, recommended: None },
        ConfigField { id: "ui.cursor_blink", path: "ui.cursor_blink", section: "ui", group: "ui", label: "Cursor blink", description: "Force blinking cursor (true) or steady block (false).", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.render_mermaid", path: "ui.render_mermaid", section: "ui", group: "ui", label: "Render mermaid", description: "Mermaid diagram handling: auto | on | off.", field_type: "enum", default: Some(jstr("auto")), options: Some(&["auto", "on", "off"]), env: None, cli: None, recommended: None },
        ConfigField { id: "ui.cancel_subagents_on_turn_cancel", path: "ui.cancel_subagents_on_turn_cancel", section: "ui", group: "ui", label: "Cancel subagents on turn cancel", description: "ask | always_stop | always_continue.", field_type: "enum", default: Some(jstr("ask")), options: Some(&["ask", "always_stop", "always_continue"]), env: None, cli: None, recommended: None },
        ConfigField { id: "ui.hunk_tracker_mode", path: "ui.hunk_tracker_mode", section: "ui", group: "ui", label: "Hunk tracker mode", description: "agent_only | all_dirty | off.", field_type: "enum", default: Some(jstr("agent_only")), options: Some(&["agent_only", "all_dirty", "off"]), env: None, cli: None, recommended: None },
        ConfigField { id: "ui.fork_secondary_model", path: "ui.fork_secondary_model", section: "ui", group: "ui", label: "Fork secondary model", description: "Model for secondary agent when forking.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.voice_keybind_enabled", path: "ui.voice_keybind_enabled", section: "ui", group: "ui", label: "Voice keybind", description: "Enable Ctrl+Space / F8 for voice dictation.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.voice_capture_mode", path: "ui.voice_capture_mode", section: "ui", group: "ui", label: "Voice capture mode", description: "hold | toggle.", field_type: "enum", default: Some(jstr("hold")), options: Some(&["hold", "toggle"]), env: None, cli: None, recommended: None },
        ConfigField { id: "ui.voice_stt_language", path: "ui.voice_stt_language", section: "ui", group: "ui", label: "Voice STT language", description: "auto or language code.", field_type: "string", default: Some(jstr("auto")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.display_refresh.auto_cadence_enabled", path: "ui.display_refresh.auto_cadence_enabled", section: "ui.display_refresh", group: "ui", label: "Display refresh auto cadence", description: "Match stream/scroll cadence to display refresh rate.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_DISPLAY_REFRESH_AUTO_CADENCE"), cli: None, recommended: None },
        ConfigField { id: "ui.contextual_hints.undo", path: "ui.contextual_hints.undo", section: "ui.contextual_hints", group: "ui", label: "Hint: undo", description: "Ctrl+Z restores a wiped prompt draft.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.contextual_hints.plan_mode", path: "ui.contextual_hints.plan_mode", section: "ui.contextual_hints", group: "ui", label: "Hint: plan mode", description: "Suggest plan mode (Shift+Tab) for planning-style prompts.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.contextual_hints.image_input", path: "ui.contextual_hints.image_input", section: "ui.contextual_hints", group: "ui", label: "Hint: image input", description: "Clipboard image paste tip when model accepts images.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "ui.contextual_hints.ssh_wrap", path: "ui.contextual_hints.ssh_wrap", section: "ui.contextual_hints", group: "ui", label: "Hint: ssh wrap", description: "Recommend grok wrap when SSH lacks a clipboard sink.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },

        // Features
        ConfigField { id: "features.memory", path: "features.memory", section: "features", group: "features", label: "Cross-session memory", description: "Enable experimental cross-session memory (legacy path; prefer [memory] enabled).", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_MEMORY"), cli: Some("--experimental-memory / --no-memory"), recommended: None },
        ConfigField { id: "features.subagents", path: "features.subagents", section: "features", group: "features", label: "Subagents", description: "Enable subagents / the task tool (legacy path; prefer [subagents] enabled).", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_SUBAGENTS"), cli: Some("--no-subagents"), recommended: None },
        ConfigField { id: "features.web_fetch", path: "features.web_fetch", section: "features", group: "features", label: "Web fetch tool", description: "Enable web_fetch (disabled by default for security).", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_WEB_FETCH"), cli: None, recommended: None },
        ConfigField { id: "features.write_file", path: "features.write_file", section: "features", group: "features", label: "Write file tool", description: "Disable with false for read-only sessions.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_WRITE_FILE"), cli: None, recommended: None },
        ConfigField { id: "features.tool_search", path: "features.tool_search", section: "features", group: "features", label: "Tool search", description: "On-demand MCP tool discovery for large toolsets.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_TOOL_SEARCH"), cli: None, recommended: None },
        ConfigField { id: "features.lsp_tools", path: "features.lsp_tools", section: "features", group: "features", label: "LSP tools", description: "Enable the LSP code-intel tool.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_LSP_TOOLS"), cli: None, recommended: None },
        ConfigField { id: "features.respect_gitignore", path: "tools.respect_gitignore", section: "tools", group: "features", label: "Respect gitignore", description: "Filter search/read tools by gitignore.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_RESPECT_GITIGNORE"), cli: None, recommended: None },
        ConfigField { id: "features.telemetry", path: "features.telemetry", section: "features", group: "privacy", label: "Telemetry feature flag", description: "Master telemetry feature toggle (when supported).", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.image_generation", path: "features.image_generation", section: "features", group: "features", label: "Image generation", description: "Enable image generation tool and slash commands.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.video_generation", path: "features.video_generation", section: "features", group: "features", label: "Video generation", description: "Enable video generation tool and slash commands.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "features.workflows", path: "features.workflows", section: "features", group: "features", label: "Workflows", description: "Enable workflows (on by default since 0.2.111).", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },

        // Memory / subagents dedicated sections
        ConfigField { id: "memory.enabled", path: "memory.enabled", section: "memory", group: "memory", label: "Memory enabled", description: "Cross-session memory master switch.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("GROK_MEMORY"), cli: Some("--experimental-memory"), recommended: None },
        ConfigField { id: "subagents.enabled", path: "subagents.enabled", section: "subagents", group: "subagents", label: "Subagents enabled", description: "Subagent / task tool master switch.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_SUBAGENTS"), cli: Some("--no-subagents"), recommended: None },

        // Skills
        ConfigField { id: "skills.paths", path: "skills.paths", section: "skills", group: "skills", label: "Skill paths", description: "Extra skill directories.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "skills.disabled", path: "skills.disabled", section: "skills", group: "skills", label: "Disabled skills", description: "Discover but do not activate.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },

        // Privacy
        ConfigField { id: "harness.disable_codebase_upload", path: "harness.disable_codebase_upload", section: "harness", group: "privacy", label: "Disable codebase upload", description: "Client request to skip whole-repo git bundle upload.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "telemetry.trace_upload", path: "telemetry.trace_upload", section: "telemetry", group: "privacy", label: "Trace upload", description: "Upload telemetry traces.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: Some(true) },

        // Auth
        ConfigField { id: "auth.oidc.client_id", path: "auth.oidc.client_id", section: "auth.oidc", group: "auth", label: "OIDC client ID", description: "Client ID for enterprise OIDC authentication.", field_type: "string", default: None, options: None, env: Some("GROK_OIDC_CLIENT_ID"), cli: None, recommended: None },
        ConfigField { id: "auth.auth_provider_command", path: "auth.auth_provider_command", section: "auth", group: "auth", label: "Auth provider command", description: "Executable that outputs a token (bare string or JSON).", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "grok_com_config.disable_api_key_auth", path: "grok_com_config.disable_api_key_auth", section: "grok_com_config", group: "auth", label: "Disable API key auth", description: "Force interactive/IdP login; disable API key usage.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "grok_com_config.force_login_team_uuid", path: "grok_com_config.force_login_team_uuid", section: "grok_com_config", group: "auth", label: "Force login team UUID", description: "Restrict login to a specific team UUID.", field_type: "string", default: None, options: None, env: None, cli: None, recommended: None },

        // Plugins
        ConfigField { id: "plugins.paths", path: "plugins.paths", section: "plugins", group: "plugins", label: "Plugin paths", description: "Extra plugin directories.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "plugins.enabled", path: "plugins.enabled", section: "plugins", group: "plugins", label: "Enabled plugins", description: "Explicitly enable plugins by name.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "plugins.disabled", path: "plugins.disabled", section: "plugins", group: "plugins", label: "Disabled plugins", description: "Discover but do not activate.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },

        // Compat Cursor / Claude
        ConfigField { id: "compat.cursor.skills", path: "compat.cursor.skills", section: "compat.cursor", group: "compat", label: "Cursor skills", description: "Scan Cursor skills directories.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CURSOR_SKILLS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.cursor.rules", path: "compat.cursor.rules", section: "compat.cursor", group: "compat", label: "Cursor rules", description: "Scan .cursor/rules/.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CURSOR_RULES_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.cursor.agents", path: "compat.cursor.agents", section: "compat.cursor", group: "compat", label: "Cursor agents", description: "Scan Cursor agent definitions.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CURSOR_AGENTS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.cursor.mcps", path: "compat.cursor.mcps", section: "compat.cursor", group: "compat", label: "Cursor MCPs", description: "Scan Cursor mcp.json.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CURSOR_MCPS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.cursor.hooks", path: "compat.cursor.hooks", section: "compat.cursor", group: "compat", label: "Cursor hooks", description: "Scan Cursor hooks.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CURSOR_HOOKS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.claude.skills", path: "compat.claude.skills", section: "compat.claude", group: "compat", label: "Claude skills", description: "Scan Claude skills.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CLAUDE_SKILLS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.claude.rules", path: "compat.claude.rules", section: "compat.claude", group: "compat", label: "Claude rules", description: "Scan Claude rules.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CLAUDE_RULES_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.claude.agents", path: "compat.claude.agents", section: "compat.claude", group: "compat", label: "Claude agents", description: "Scan CLAUDE.md / CLAUDE.local.md.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CLAUDE_AGENTS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.claude.mcps", path: "compat.claude.mcps", section: "compat.claude", group: "compat", label: "Claude MCPs", description: "Scan Claude MCP config.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CLAUDE_MCPS_ENABLED"), cli: None, recommended: None },
        ConfigField { id: "compat.claude.hooks", path: "compat.claude.hooks", section: "compat.claude", group: "compat", label: "Claude hooks", description: "Scan Claude hooks.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("GROK_CLAUDE_HOOKS_ENABLED"), cli: None, recommended: None },

        // MCP sample
        ConfigField { id: "mcp.id", path: "mcp_servers.sample.id", section: "mcp_servers", group: "mcp", label: "MCP server id", description: "Table name under [mcp_servers.id].", field_type: "string", default: Some(jstr("filesystem")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.enabled", path: "mcp_servers.sample.enabled", section: "mcp_servers", group: "mcp", label: "MCP enabled", description: "Enable this MCP server entry.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.command", path: "mcp_servers.sample.command", section: "mcp_servers", group: "mcp", label: "MCP command", description: "Stdio command (e.g. npx).", field_type: "string", default: Some(jstr("npx")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.args", path: "mcp_servers.sample.args", section: "mcp_servers", group: "mcp", label: "MCP args", description: "Comma-separated args (e.g. -y,@modelcontextprotocol/server-filesystem,/path).", field_type: "string-list", default: Some(serde_json::json!(["-y", "@modelcontextprotocol/server-filesystem", "/path/to/allowed"])), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.startup_timeout_sec", path: "mcp_servers.sample.startup_timeout_sec", section: "mcp_servers", group: "mcp", label: "MCP startup timeout sec", description: "Handshake timeout in seconds.", field_type: "number", default: Some(jnum(30.0)), options: None, env: Some("GROK_MCP_STARTUP_TIMEOUT_SECS"), cli: None, recommended: None },
        ConfigField { id: "mcp.tool_timeout_sec", path: "mcp_servers.sample.tool_timeout_sec", section: "mcp_servers", group: "mcp", label: "MCP tool timeout sec", description: "Per-tool timeout in seconds.", field_type: "number", default: Some(jnum(6000.0)), options: None, env: None, cli: None, recommended: None },
    ]
}

pub fn cli_flags() -> &'static [CliFlag] {
    &[
        CliFlag { flag: "--cwd <PATH>", description: "Set working directory", category: "Session" },
        CliFlag { flag: "-r, --resume [<ID>]", description: "Resume session (omit ID = most recent)", category: "Session" },
        CliFlag { flag: "-c, --continue", description: "Continue most recent session for this directory", category: "Session" },
        CliFlag { flag: "-s, --session-id <UUID>", description: "Use a specific UUID for a new session", category: "Session" },
        CliFlag { flag: "--fork-session", description: "When resuming, fork into a new session ID", category: "Session" },
        CliFlag { flag: "-w, --worktree [<NAME>]", description: "Start session in a new git worktree", category: "Session" },
        CliFlag { flag: "--ref <REF>", description: "Branch, tag, or commit for the worktree", category: "Session" },
        CliFlag { flag: "-m, --model <MODEL>", description: "Model ID", category: "Model" },
        CliFlag { flag: "--effort <LEVEL>", description: "Reasoning effort level", category: "Model" },
        CliFlag { flag: "--always-approve / --yolo", description: "Auto-approve all tool executions", category: "Permissions" },
        CliFlag { flag: "--allow <RULE>", description: "Permission allow rule (e.g. 'Bash(git *)')", category: "Permissions" },
        CliFlag { flag: "--deny <RULE>", description: "Permission deny rule", category: "Permissions" },
        CliFlag { flag: "--sandbox <PROFILE>", description: "Sandbox profile: off | workspace | read-only | strict", category: "Permissions" },
        CliFlag { flag: "--rules <TEXT>", description: "Append extra rules to system prompt", category: "Prompt" },
        CliFlag { flag: "--system-prompt-override <TEXT>", description: "Replace system prompt entirely", category: "Prompt" },
        CliFlag { flag: "--tools <LIST>", description: "Allow only these built-in tools", category: "Tools" },
        CliFlag { flag: "--disallowed-tools <LIST>", description: "Disallow built-in tools", category: "Tools" },
        CliFlag { flag: "--max-turns <N>", description: "Maximum agent turns", category: "Agent" },
        CliFlag { flag: "--no-plan", description: "Disable planning", category: "Agent" },
        CliFlag { flag: "--no-subagents", description: "Disable subagents", category: "Agent" },
        CliFlag { flag: "--no-memory", description: "Disable memory", category: "Agent" },
        CliFlag { flag: "--disable-web-search", description: "Disable web search", category: "Agent" },
        CliFlag { flag: "--experimental-memory", description: "Enable cross-session memory", category: "Agent" },
        CliFlag { flag: "--oauth", description: "Use OAuth when welcome screen starts auth", category: "Auth" },
        CliFlag { flag: "-p, --single <PROMPT>", description: "Headless one-shot prompt", category: "Headless" },
        CliFlag { flag: "--output-format <FMT>", description: "plain | json | streaming-json", category: "Headless" },
        CliFlag { flag: "--no-alt-screen", description: "Inline (no fullscreen TUI takeover)", category: "Headless" },
        CliFlag { flag: "--no-auto-update", description: "Skip background update checks", category: "Headless" },
        CliFlag { flag: "--allowedTools", description: "Claude Code alias → --tools", category: "Aliases" },
        CliFlag { flag: "--disallowedTools", description: "Claude Code alias → --disallowed-tools", category: "Aliases" },
        CliFlag { flag: "--append-system-prompt", description: "Claude Code alias → --rules", category: "Aliases" },
        CliFlag { flag: "--system-prompt", description: "Claude Code alias → --system-prompt-override", category: "Aliases" },
        CliFlag { flag: "--dangerously-skip-permissions", description: "Claude Code alias → --always-approve", category: "Aliases" },
    ]
}

pub fn env_vars() -> &'static [EnvVar] {
    &[
        EnvVar { name: "GROK_HOME", description: "Home for config, auth, sessions (default ~/.grok)", category: "Paths" },
        EnvVar { name: "XAI_API_KEY", description: "API key for headless / CI auth", category: "Auth" },
        EnvVar { name: "GROK_DEFAULT_MODEL", description: "Session default model (= -m)", category: "Models" },
        EnvVar { name: "GROK_WEB_SEARCH_MODEL", description: "Model for web_search tool", category: "Models" },
        EnvVar { name: "GROK_MODELS_BASE_URL", description: "Custom inference base URL", category: "Models" },
        EnvVar { name: "GROK_MODELS_LIST_URL", description: "Override model-list URL", category: "Models" },
        EnvVar { name: "GROK_XAI_API_BASE_URL", description: "xAI API base (default https://api.x.ai/v1)", category: "Models" },
        EnvVar { name: "GROK_SANDBOX", description: "Sandbox profile name", category: "Sandbox" },
        EnvVar { name: "GROK_SANDBOX_AUTO_ALLOW_BASH", description: "1/0 auto-allow bash in sandbox", category: "Sandbox" },
        EnvVar { name: "GROK_DISABLE_AUTOUPDATER", description: "Suppress auto-updater when set", category: "CLI" },
        EnvVar { name: "GROK_MEMORY", description: "1/0 cross-session memory", category: "Features" },
        EnvVar { name: "GROK_SUBAGENTS", description: "1/0 subagents / task tool", category: "Features" },
        EnvVar { name: "GROK_AGENT", description: "Built-in agent name, profile, or path", category: "Features" },
        EnvVar { name: "GROK_WEB_FETCH", description: "1/0 enable web_fetch", category: "Tools" },
        EnvVar { name: "GROK_WEB_FETCH_PROXY", description: "Egress proxy URL for web_fetch", category: "Tools" },
        EnvVar { name: "GROK_WRITE_FILE", description: "0 disables write tool", category: "Tools" },
        EnvVar { name: "GROK_TOOL_SEARCH", description: "1/0 MCP tool discovery", category: "Tools" },
        EnvVar { name: "GROK_LSP_TOOLS", description: "1/0 LSP code-intel", category: "Tools" },
        EnvVar { name: "GROK_RESPECT_GITIGNORE", description: "1/0 force gitignore filtering", category: "Tools" },
        EnvVar { name: "GROK_THEME", description: "Color theme", category: "UI" },
        EnvVar { name: "GROK_SHOW_THINKING_BLOCKS", description: "1/0 reasoning blocks", category: "UI" },
        EnvVar { name: "GROK_GROUP_TOOL_VERBS", description: "1/0 fold tool rows", category: "UI" },
        EnvVar { name: "GROK_COLLAPSED_EDIT_BLOCKS", description: "1/0 collapse edits", category: "UI" },
        EnvVar { name: "GROK_PROMPT_SUGGESTIONS", description: "1/0 ghost suggestions", category: "UI" },
        EnvVar { name: "GROK_SCROLL_SPEED", description: "1–100 scroll speed", category: "UI" },
        EnvVar { name: "GROK_SCROLL_MODE", description: "auto | wheel | trackpad", category: "UI" },
        EnvVar { name: "GROK_SCROLL_LINES", description: "Lines per tick 1–10", category: "UI" },
        EnvVar { name: "GROK_INVERT_SCROLL", description: "1/0 invert scroll", category: "UI" },
        EnvVar { name: "GROK_DEFAULT_SELECTED_PERMISSION", description: "First prompt default row", category: "Permissions" },
        EnvVar { name: "GROK_REMEMBER_TOOL_APPROVALS", description: "1/0 always-allow options", category: "Permissions" },
        EnvVar { name: "GROK_MOUSE_REPORTING_TOGGLE", description: "1/0 Ctrl+R mouse toggle", category: "UI" },
        EnvVar { name: "GROK_DISPLAY_REFRESH_AUTO_CADENCE", description: "1/0 match display refresh", category: "UI" },
        EnvVar { name: "GROK_MCP_STARTUP_TIMEOUT_SECS", description: "MCP handshake timeout (default 30)", category: "MCP" },
        EnvVar { name: "MCP_TIMEOUT", description: "Claude-compatible MCP timeout (ms)", category: "MCP" },
        EnvVar { name: "GROK_LOG_FILE", description: "Log file path", category: "Logging" },
        EnvVar { name: "RUST_LOG", description: "Log filter e.g. debug", category: "Logging" },
        EnvVar { name: "GROK_CRASH_HANDLER", description: "1/0 crash reports under $GROK_HOME/crash", category: "Logging" },
        EnvVar { name: "HTTPS_PROXY", description: "HTTPS proxy", category: "Network" },
        EnvVar { name: "HTTP_PROXY", description: "HTTP proxy", category: "Network" },
        EnvVar { name: "NO_PROXY", description: "Proxy bypass list", category: "Network" },
        EnvVar { name: "GROK_OIDC_ISSUER", description: "Enterprise OIDC issuer", category: "Auth" },
        EnvVar { name: "GROK_OIDC_CLIENT_ID", description: "Enterprise OIDC client", category: "Auth" },
        EnvVar { name: "GROK_POOL_IDLE_TIMEOUT_SECS", description: "HTTP pool idle (default 90)", category: "Network" },
        EnvVar { name: "GROK_EXTRA_CA_BUNDLE", description: "Extra TLS roots (v0.2.117+)", category: "Network" },
        EnvVar { name: "GROK_CURSOR_SKILLS_ENABLED", description: "Scan Cursor skills", category: "Compat" },
        EnvVar { name: "GROK_CURSOR_RULES_ENABLED", description: "Scan .cursor/rules", category: "Compat" },
        EnvVar { name: "GROK_CURSOR_AGENTS_ENABLED", description: "Scan Cursor agents", category: "Compat" },
        EnvVar { name: "GROK_CURSOR_MCPS_ENABLED", description: "Scan Cursor mcp.json", category: "Compat" },
        EnvVar { name: "GROK_CURSOR_HOOKS_ENABLED", description: "Scan Cursor hooks", category: "Compat" },
        EnvVar { name: "GROK_CLAUDE_SKILLS_ENABLED", description: "Scan Claude skills", category: "Compat" },
        EnvVar { name: "GROK_CLAUDE_RULES_ENABLED", description: "Scan Claude rules", category: "Compat" },
        EnvVar { name: "GROK_CLAUDE_AGENTS_ENABLED", description: "Scan CLAUDE.md", category: "Compat" },
        EnvVar { name: "GROK_CLAUDE_MCPS_ENABLED", description: "Scan Claude MCP config", category: "Compat" },
        EnvVar { name: "GROK_CLAUDE_HOOKS_ENABLED", description: "Scan Claude hooks", category: "Compat" },
    ]
}

pub fn subcommands() -> &'static [Subcommand] {
    &[
        Subcommand { cmd: "grok login [--device-auth]", desc: "Sign in (device-code for headless)" },
        Subcommand { cmd: "grok logout", desc: "Sign out and clear credentials" },
        Subcommand { cmd: "grok inspect [--json]", desc: "Show discovered config for current directory" },
        Subcommand { cmd: "grok models", desc: "List available models" },
        Subcommand { cmd: "grok mcp <list|add|remove|doctor>", desc: "Manage MCP servers" },
        Subcommand { cmd: "grok plugin <…>", desc: "Install/enable/disable plugins" },
        Subcommand { cmd: "grok plugin marketplace <…>", desc: "Manage marketplace sources" },
        Subcommand { cmd: "grok sessions <list|search|delete>", desc: "Manage sessions" },
        Subcommand { cmd: "grok export <id> [out]", desc: "Export session as Markdown" },
        Subcommand { cmd: "grok import [targets…]", desc: "Import sessions from Claude Code" },
        Subcommand { cmd: "grok memory clear [--workspace|--global|--all]", desc: "Clear memory files" },
        Subcommand { cmd: "grok worktree <list|show|rm|gc>", desc: "Manage session worktrees" },
        Subcommand { cmd: "grok dashboard", desc: "Open Agent Dashboard" },
        Subcommand { cmd: "grok agent stdio", desc: "Run as ACP agent over stdin/stdout" },
        Subcommand { cmd: "grok wrap <command…>", desc: "PTY with OSC 52 clipboard forwarding" },
        Subcommand { cmd: "grok update [--check|--stable|--alpha]", desc: "Update CLI" },
        Subcommand { cmd: "grok version", desc: "Print version" },
        Subcommand { cmd: "grok completions <shell>", desc: "Shell completion scripts" },
        Subcommand { cmd: "grok setup", desc: "Fetch and install managed configuration" },
    ]
}

pub fn presets() -> Vec<Preset> {
    vec![
        Preset {
            id: "privacy-first",
            name: "Privacy first",
            description: "Disable codebase upload & telemetry; sandbox workspace.",
            enabled: &["harness.disable_codebase_upload", "telemetry.trace_upload", "features.telemetry", "sandbox.profile", "models.default"],
            values: serde_json::json!({
                "harness.disable_codebase_upload": true,
                "telemetry.trace_upload": false,
                "features.telemetry": false,
                "sandbox.profile": "workspace",
                "models.default": "grok-build"
            }),
        },
        Preset {
            id: "safe-dev",
            name: "Safe development",
            description: "Workspace sandbox, ask mode, no web_fetch, gitignore on.",
            enabled: &["sandbox.profile", "permissions.permission_mode", "features.web_fetch", "features.respect_gitignore", "features.write_file", "models.default", "harness.disable_codebase_upload"],
            values: serde_json::json!({
                "sandbox.profile": "workspace",
                "permissions.permission_mode": "ask",
                "features.web_fetch": false,
                "features.respect_gitignore": true,
                "features.write_file": true,
                "models.default": "grok-build",
                "harness.disable_codebase_upload": true
            }),
        },
        Preset {
            id: "ci-headless",
            name: "CI / headless",
            description: "Always-approve, no auto-update, streaming-friendly defaults.",
            enabled: &["permissions.permission_mode", "cli.auto_update", "models.default", "features.subagents", "sandbox.profile", "subagents.enabled"],
            values: serde_json::json!({
                "permissions.permission_mode": "always-approve",
                "cli.auto_update": false,
                "models.default": "grok-build",
                "features.subagents": true,
                "subagents.enabled": true,
                "sandbox.profile": "workspace"
            }),
        },
        Preset {
            id: "read-only-review",
            name: "Read-only review",
            description: "Strict/read-only sandbox, no write tool.",
            enabled: &["sandbox.profile", "features.write_file", "permissions.permission_mode", "models.default"],
            values: serde_json::json!({
                "sandbox.profile": "read-only",
                "features.write_file": false,
                "permissions.permission_mode": "ask",
                "models.default": "grok-build"
            }),
        },
        Preset {
            id: "power-ui",
            name: "Power UI",
            description: "Thinking blocks, timeline, vim scrollback, voice, mermaid on.",
            enabled: &[
                "models.default", "ui.show_thinking_blocks", "ui.show_timeline", "ui.vim_mode",
                "ui.render_mermaid", "ui.voice_keybind_enabled", "ui.prompt_suggestions",
                "ui.group_tool_verbs", "features.lsp_tools", "subagents.enabled",
            ],
            values: serde_json::json!({
                "models.default": "grok-build",
                "ui.show_thinking_blocks": true,
                "ui.show_timeline": true,
                "ui.vim_mode": true,
                "ui.render_mermaid": "on",
                "ui.voice_keybind_enabled": true,
                "ui.prompt_suggestions": true,
                "ui.group_tool_verbs": true,
                "features.lsp_tools": true,
                "subagents.enabled": true
            }),
        },
    ]
}

pub fn payload() -> SchemaPayload {
    SchemaPayload {
        groups: groups(),
        fields: fields(),
        cli_flags: cli_flags(),
        env_vars: env_vars(),
        subcommands: subcommands(),
        presets: presets(),
        version_note: "Grok Build CLI · documented track v0.2.117 · Grok 4.5 · exhaustive knobs · pure Rust builder",
    }
}
