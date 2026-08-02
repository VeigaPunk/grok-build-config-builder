//! OpenCode Titanium — wild but coherent config builder.
//! Target: opencode.json / ~/.config/opencode/opencode.json
//! Exhaustive knobs from OpenCode docs (config + CLI + env).

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
        version_note: "OpenCode · Titanium build · opencode.json · pure Rust builder · exhaustive knobs",
    }
}

fn groups() -> &'static [FieldGroup] {
    &[
        FieldGroup { id: "models", title: "Models", description: "Primary + small models and provider allowlists." },
        FieldGroup { id: "agents", title: "Agents", description: "Default agent, depth, plan mode." },
        FieldGroup { id: "tools", title: "Tools & permissions", description: "write/bash/edit/read/grep gates and tool toggles." },
        FieldGroup { id: "compaction", title: "Compaction", description: "Context auto-compact settings." },
        FieldGroup { id: "server", title: "Server", description: "opencode serve / web." },
        FieldGroup { id: "ux", title: "UX", description: "Theme, share, autoupdate, snapshot, shell, instructions." },
        FieldGroup { id: "tui", title: "TUI", description: "Scroll, mouse, attention/notifications, diff style." },
        FieldGroup { id: "experimental", title: "Experimental", description: "Plan mode, background subagents, scout, workspaces, hooks." },
        FieldGroup { id: "lsp", title: "LSP & formatters", description: "Language intelligence and format-on-write." },
        FieldGroup { id: "mcp", title: "MCP sample", description: "Optional remote MCP server stub." },
        FieldGroup { id: "attachment", title: "Attachments", description: "Image attachment resize limits." },
        FieldGroup { id: "watcher", title: "File watcher", description: "Ignore globs for the watcher." },
        FieldGroup { id: "plugins", title: "Plugins", description: "Plugin modules to load." },
        FieldGroup { id: "provider", title: "Provider options", description: "Provider timeout / cache options." },
    ]
}

fn fields() -> Vec<ConfigField> {
    vec![
        ConfigField { id: "model", path: "model", section: "", group: "models", label: "Primary model", description: "provider/model id, e.g. anthropic/claude-sonnet-4-6 or openai/gpt-5.4.", field_type: "string", default: Some(jstr("anthropic/claude-sonnet-4-6")), options: None, env: None, cli: Some("-m / --model"), recommended: Some(true) },
        ConfigField { id: "small_model", path: "small_model", section: "", group: "models", label: "Small model", description: "Cheap model for titles / light tasks.", field_type: "string", default: Some(jstr("anthropic/claude-haiku-4-5")), options: None, env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "enabled_providers", path: "enabled_providers", section: "", group: "models", label: "Enabled providers", description: "Comma-separated allow-list of provider ids.", field_type: "string-list", default: Some(json!(["anthropic", "openai", "openrouter"])), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "disabled_providers", path: "disabled_providers", section: "", group: "models", label: "Disabled providers", description: "Comma-separated deny-list.", field_type: "string-list", default: None, options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "default_agent", path: "default_agent", section: "", group: "agents", label: "Default agent", description: "build | plan | or custom agent id.", field_type: "string", default: Some(jstr("build")), options: None, env: None, cli: Some("--agent"), recommended: Some(true) },
        ConfigField { id: "subagent_depth", path: "subagent_depth", section: "", group: "agents", label: "Subagent depth", description: "Max nesting (0 disables).", field_type: "number", default: Some(jnum(2.0)), options: None, env: None, cli: None, recommended: Some(true) },

        ConfigField { id: "tools.write", path: "tools.write", section: "tools", group: "tools", label: "Write tool", description: "Allow write tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.bash", path: "tools.bash", section: "tools", group: "tools", label: "Bash tool", description: "Allow bash tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.edit", path: "tools.edit", section: "tools", group: "tools", label: "Edit tool", description: "Allow edit tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.read", path: "tools.read", section: "tools", group: "tools", label: "Read tool", description: "Allow read tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.grep", path: "tools.grep", section: "tools", group: "tools", label: "Grep tool", description: "Allow grep/search tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.glob", path: "tools.glob", section: "tools", group: "tools", label: "Glob tool", description: "Allow glob/file-find tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "tools.webfetch", path: "tools.webfetch", section: "tools", group: "tools", label: "Web fetch tool", description: "Allow webfetch tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "edit", path: "edit", section: "", group: "tools", label: "Edit permission", description: "allow | ask | deny for edits.", field_type: "enum", default: Some(jstr("allow")), options: Some(&["allow", "ask", "deny"]), env: None, cli: None, recommended: Some(true) },
        ConfigField { id: "bash", path: "bash", section: "", group: "tools", label: "Bash permission", description: "allow | ask | deny for bash.", field_type: "enum", default: Some(jstr("ask")), options: Some(&["allow", "ask", "deny"]), env: Some("OPENCODE_PERMISSION"), cli: Some("--dangerously-skip-permissions / --auto"), recommended: Some(true) },

        ConfigField { id: "compaction.auto", path: "compaction.auto", section: "compaction", group: "compaction", label: "Auto compact", description: "Automatically compact long contexts.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_DISABLE_AUTOCOMPACT"), cli: None, recommended: Some(true) },
        ConfigField { id: "compaction.prune", path: "compaction.prune", section: "compaction", group: "compaction", label: "Prune on compact", description: "Prune old turns when compacting.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "compaction.reserved", path: "compaction.reserved", section: "compaction", group: "compaction", label: "Reserved tokens", description: "Token buffer reserved during compact.", field_type: "number", default: Some(jnum(12_000.0)), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "server.port", path: "server.port", section: "server", group: "server", label: "Server port", description: "Port for opencode serve / web.", field_type: "number", default: Some(jnum(4096.0)), options: None, env: None, cli: Some("--port"), recommended: None },
        ConfigField { id: "server.hostname", path: "server.hostname", section: "server", group: "server", label: "Server hostname", description: "Bind host.", field_type: "string", default: Some(jstr("127.0.0.1")), options: None, env: None, cli: Some("--hostname"), recommended: None },
        ConfigField { id: "server.mdns", path: "server.mdns", section: "server", group: "server", label: "mDNS", description: "Advertise via mDNS.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: Some("--mdns"), recommended: None },
        ConfigField { id: "server.mdnsDomain", path: "server.mdnsDomain", section: "server", group: "server", label: "mDNS domain", description: "Custom mDNS domain (default opencode.local).", field_type: "string", default: Some(jstr("opencode.local")), options: None, env: None, cli: Some("--mdns-domain"), recommended: None },
        ConfigField { id: "server.cors", path: "server.cors", section: "server", group: "server", label: "CORS origins", description: "Comma-separated allowed CORS origins.", field_type: "string-list", default: None, options: None, env: None, cli: Some("--cors"), recommended: None },

        ConfigField { id: "theme", path: "theme", section: "", group: "ux", label: "Theme", description: "TUI theme (system or named).", field_type: "string", default: Some(jstr("system")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "share", path: "share", section: "", group: "ux", label: "Share mode", description: "manual | auto | disabled.", field_type: "enum", default: Some(jstr("manual")), options: Some(&["manual", "auto", "disabled"]), env: Some("OPENCODE_AUTO_SHARE"), cli: Some("--share"), recommended: None },
        ConfigField { id: "autoshare", path: "autoshare", section: "", group: "ux", label: "Autoshare", description: "Auto-share every session.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("OPENCODE_AUTO_SHARE"), cli: None, recommended: None },
        ConfigField { id: "autoupdate", path: "autoupdate", section: "", group: "ux", label: "Autoupdate", description: "true | false | notify.", field_type: "enum", default: Some(jstr("notify")), options: Some(&["true", "false", "notify"]), env: Some("OPENCODE_DISABLE_AUTOUPDATE"), cli: None, recommended: None },
        ConfigField { id: "snapshot", path: "snapshot", section: "", group: "ux", label: "Snapshot tracking", description: "Track file changes during session.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "shell", path: "shell", section: "", group: "ux", label: "Shell", description: "Shell binary for tools (empty = auto).", field_type: "string", default: None, options: None, env: Some("OPENCODE_GIT_BASH_PATH"), cli: None, recommended: None },
        ConfigField { id: "username", path: "username", section: "", group: "ux", label: "Username", description: "Display name in TUI.", field_type: "string", default: Some(jstr("titanium")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "instructions", path: "instructions", section: "", group: "ux", label: "Instruction files", description: "Paths/globs for extra instruction markdown (comma-separated).", field_type: "string-list", default: Some(json!(["AGENTS.md", "CONTRIBUTING.md"])), options: None, env: None, cli: None, recommended: Some(true) },

        // TUI
        ConfigField { id: "scroll_speed", path: "scroll_speed", section: "", group: "tui", label: "Scroll speed", description: "Lines to scroll per tick.", field_type: "number", default: Some(jnum(3.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mouse", path: "mouse", section: "", group: "tui", label: "Mouse support", description: "Enable mouse input in TUI.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_DISABLE_MOUSE"), cli: None, recommended: None },
        ConfigField { id: "diff_style", path: "diff_style", section: "", group: "tui", label: "Diff style", description: "auto or named diff display style.", field_type: "string", default: Some(jstr("auto")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attention.enabled", path: "attention.enabled", section: "attention", group: "tui", label: "Attention enabled", description: "Desktop notifications / sound master switch.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attention.notifications", path: "attention.notifications", section: "attention", group: "tui", label: "Desktop notifications", description: "Show desktop notifications when attention needed.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attention.sound", path: "attention.sound", section: "attention", group: "tui", label: "Attention sound", description: "Play sound on attention events.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attention.volume", path: "attention.volume", section: "attention", group: "tui", label: "Attention volume", description: "Notification sound volume 0–1.", field_type: "number", default: Some(jnum(0.5)), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "experimental.plan_mode", path: "experimental.plan_mode", section: "experimental", group: "experimental", label: "Plan mode", description: "Enable experimental plan mode.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_PLAN_MODE"), cli: None, recommended: Some(true) },
        ConfigField { id: "experimental.background_subagents", path: "experimental.background_subagents", section: "experimental", group: "experimental", label: "Background subagents", description: "Run subagents in background.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_BACKGROUND_SUBAGENTS"), cli: None, recommended: Some(true) },
        ConfigField { id: "experimental.scout", path: "experimental.scout", section: "experimental", group: "experimental", label: "Scout subagent", description: "Enable Scout research subagent.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_SCOUT"), cli: None, recommended: Some(true) },
        ConfigField { id: "experimental.workspaces", path: "experimental.workspaces", section: "experimental", group: "experimental", label: "Workspaces", description: "Enable workspace support.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_WORKSPACES"), cli: None, recommended: None },
        ConfigField { id: "experimental.hooks", path: "experimental.hooks", section: "experimental", group: "experimental", label: "Hooks", description: "Experimental hooks system.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "experimental.lsp_tool", path: "experimental.lsp_tool", section: "experimental", group: "experimental", label: "LSP tool", description: "Expose LSP as a tool.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_LSP_TOOL"), cli: None, recommended: None },
        ConfigField { id: "experimental.parallel", path: "experimental.parallel", section: "experimental", group: "experimental", label: "Parallel search", description: "Parallel web search execution.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_PARALLEL"), cli: None, recommended: None },
        ConfigField { id: "experimental.filewatcher", path: "experimental.filewatcher", section: "experimental", group: "experimental", label: "File watcher", description: "Experimental file-watcher integration.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_FILEWATCHER"), cli: None, recommended: None },
        ConfigField { id: "experimental.markdown", path: "experimental.markdown", section: "experimental", group: "experimental", label: "Markdown improvements", description: "Experimental markdown processing.", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_MARKDOWN"), cli: None, recommended: None },
        ConfigField { id: "experimental.icon_discovery", path: "experimental.icon_discovery", section: "experimental", group: "experimental", label: "Icon discovery", description: "Icon detection in TUI.", field_type: "boolean", default: Some(jbool(false)), options: None, env: Some("OPENCODE_EXPERIMENTAL_ICON_DISCOVERY"), cli: None, recommended: None },
        ConfigField { id: "experimental.bash_default_timeout_ms", path: "experimental.bash_default_timeout_ms", section: "experimental", group: "experimental", label: "Bash default timeout ms", description: "Default bash command timeout.", field_type: "number", default: Some(jnum(120000.0)), options: None, env: Some("OPENCODE_EXPERIMENTAL_BASH_DEFAULT_TIMEOUT_MS"), cli: None, recommended: None },

        ConfigField { id: "lsp", path: "lsp", section: "", group: "lsp", label: "LSP enabled", description: "Enable language servers (boolean master switch).", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_DISABLE_LSP_DOWNLOAD"), cli: None, recommended: Some(true) },
        ConfigField { id: "formatter", path: "formatter", section: "", group: "lsp", label: "Formatters enabled", description: "Enable code formatters (boolean master switch).", field_type: "boolean", default: Some(jbool(true)), options: None, env: Some("OPENCODE_EXPERIMENTAL_OXFMT"), cli: None, recommended: None },

        ConfigField { id: "mcp.demo.enabled", path: "mcp.demo.enabled", section: "mcp", group: "mcp", label: "Demo MCP enabled", description: "Include a sample remote MCP entry.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.demo.url", path: "mcp.demo.url", section: "mcp", group: "mcp", label: "Demo MCP URL", description: "Remote MCP server URL.", field_type: "string", default: Some(jstr("https://mcp.example.com/sse")), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "mcp.demo.type", path: "mcp.demo.type", section: "mcp", group: "mcp", label: "Demo MCP type", description: "remote | local.", field_type: "enum", default: Some(jstr("remote")), options: Some(&["remote", "local"]), env: None, cli: None, recommended: None },

        ConfigField { id: "attachment.image.auto_resize", path: "attachment.image.auto_resize", section: "attachment", group: "attachment", label: "Auto-resize images", description: "Resize oversized images instead of rejecting.", field_type: "boolean", default: Some(jbool(true)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attachment.image.max_width", path: "attachment.image.max_width", section: "attachment", group: "attachment", label: "Image max width", description: "Max width in pixels.", field_type: "number", default: Some(jnum(2000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attachment.image.max_height", path: "attachment.image.max_height", section: "attachment", group: "attachment", label: "Image max height", description: "Max height in pixels.", field_type: "number", default: Some(jnum(2000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "attachment.image.max_base64_bytes", path: "attachment.image.max_base64_bytes", section: "attachment", group: "attachment", label: "Image max base64 bytes", description: "Max base64 payload size (~5MB default).", field_type: "number", default: Some(jnum(5242880.0)), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "watcher.ignore", path: "watcher.ignore", section: "watcher", group: "watcher", label: "Watcher ignore globs", description: "Comma-separated globs to ignore.", field_type: "string-list", default: Some(json!(["node_modules/**", "dist/**", ".git/**"])), options: None, env: None, cli: None, recommended: None },

        ConfigField { id: "plugin", path: "plugin", section: "", group: "plugins", label: "Plugins", description: "Plugin module names or paths (comma-separated).", field_type: "string-list", default: None, options: None, env: Some("OPENCODE_DISABLE_DEFAULT_PLUGINS"), cli: Some("--pure"), recommended: None },

        ConfigField { id: "provider.timeout", path: "provider.options.timeout", section: "provider", group: "provider", label: "Provider timeout ms", description: "Default provider request timeout (false disables).", field_type: "number", default: Some(jnum(300000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.chunkTimeout", path: "provider.options.chunkTimeout", section: "provider", group: "provider", label: "Chunk timeout ms", description: "Abort if no stream chunk arrives.", field_type: "number", default: Some(jnum(30000.0)), options: None, env: None, cli: None, recommended: None },
        ConfigField { id: "provider.setCacheKey", path: "provider.options.setCacheKey", section: "provider", group: "provider", label: "Set cache key", description: "Always set a cache key for the provider.", field_type: "boolean", default: Some(jbool(false)), options: None, env: None, cli: None, recommended: None },
    ]
}

fn cli_flags() -> &'static [CliFlag] {
    &[
        CliFlag { flag: "opencode", description: "Start TUI", category: "Session" },
        CliFlag { flag: "opencode run \"<prompt>\"", description: "Headless run", category: "Session" },
        CliFlag { flag: "opencode serve", description: "Headless API server", category: "Server" },
        CliFlag { flag: "opencode web", description: "Server + web UI", category: "Server" },
        CliFlag { flag: "opencode attach <url>", description: "Attach TUI to remote server", category: "Server" },
        CliFlag { flag: "-m / --model <MODEL>", description: "Choose provider/model", category: "Model" },
        CliFlag { flag: "--variant <LEVEL>", description: "Provider-specific reasoning effort", category: "Model" },
        CliFlag { flag: "--thinking", description: "Show thinking blocks in TUI", category: "UX" },
        CliFlag { flag: "-c / --continue", description: "Continue last session", category: "Session" },
        CliFlag { flag: "-s / --session <ID>", description: "Continue a specific session", category: "Session" },
        CliFlag { flag: "--fork", description: "Fork session when continuing", category: "Session" },
        CliFlag { flag: "--prompt <TEXT>", description: "Seed agent with prompt at launch", category: "Session" },
        CliFlag { flag: "--agent <NAME>", description: "Launch as named agent", category: "Agents" },
        CliFlag { flag: "--share", description: "Make session shareable", category: "UX" },
        CliFlag { flag: "-f / --file <PATH>", description: "Attach files to initial prompt", category: "Session" },
        CliFlag { flag: "--title <TITLE>", description: "Title for new session", category: "Session" },
        CliFlag { flag: "--dir <PATH>", description: "Working directory", category: "Session" },
        CliFlag { flag: "--port <N>", description: "Local server port", category: "Server" },
        CliFlag { flag: "--hostname <HOST>", description: "Bind hostname", category: "Server" },
        CliFlag { flag: "--mdns", description: "Enable mDNS discovery", category: "Server" },
        CliFlag { flag: "--mdns-domain <DOMAIN>", description: "Custom mDNS domain", category: "Server" },
        CliFlag { flag: "--cors <ORIGIN>", description: "Additional CORS origin", category: "Server" },
        CliFlag { flag: "--format <default|json>", description: "Output format for run", category: "Headless" },
        CliFlag { flag: "--auto", description: "Auto-approve permissions that are not explicitly denied", category: "Permissions" },
        CliFlag { flag: "--dangerously-skip-permissions", description: "Auto-approve every permission", category: "Permissions" },
        CliFlag { flag: "--pure", description: "No external plugins", category: "Global" },
        CliFlag { flag: "--print-logs", description: "Logs to stderr", category: "Global" },
        CliFlag { flag: "--log-level <LEVEL>", description: "DEBUG|INFO|WARN|ERROR", category: "Global" },
        CliFlag { flag: "-v / --version", description: "Print version", category: "Global" },
        CliFlag { flag: "--password / -p", description: "Basic auth password (attach/run)", category: "Server" },
        CliFlag { flag: "--username / -u", description: "Basic auth username (attach/run)", category: "Server" },
        CliFlag { flag: "--attach <url>", description: "Attach run to a remote server", category: "Server" },
        CliFlag { flag: "--command <name>", description: "Run a named custom command", category: "Session" },
    ]
}

fn env_vars() -> &'static [EnvVar] {
    &[
        EnvVar { name: "OPENCODE_CONFIG", description: "Path to config file", category: "Paths" },
        EnvVar { name: "OPENCODE_CONFIG_DIR", description: "Config directory", category: "Paths" },
        EnvVar { name: "OPENCODE_CONFIG_CONTENT", description: "Inline JSON overrides", category: "Paths" },
        EnvVar { name: "OPENCODE_TUI_CONFIG", description: "Path to TUI config", category: "Paths" },
        EnvVar { name: "OPENCODE_PERMISSION", description: "Inline JSON permissions array", category: "Permissions" },
        EnvVar { name: "OPENCODE_AUTO_SHARE", description: "Auto-share every session", category: "UX" },
        EnvVar { name: "OPENCODE_DISABLE_AUTOCOMPACT", description: "Disable auto compact", category: "Context" },
        EnvVar { name: "OPENCODE_DISABLE_AUTOUPDATE", description: "Disable update checks", category: "UX" },
        EnvVar { name: "OPENCODE_DISABLE_MOUSE", description: "Disable TUI mouse input", category: "UX" },
        EnvVar { name: "OPENCODE_DISABLE_LSP_DOWNLOAD", description: "Skip auto-download of language servers", category: "LSP" },
        EnvVar { name: "OPENCODE_DISABLE_DEFAULT_PLUGINS", description: "Disable bundled plugins", category: "Plugins" },
        EnvVar { name: "OPENCODE_DISABLE_MODELS_FETCH", description: "Skip remote models.dev fetch", category: "Models" },
        EnvVar { name: "OPENCODE_ENABLE_EXA", description: "Enable Exa web search", category: "Tools" },
        EnvVar { name: "OPENCODE_ENABLE_EXPERIMENTAL_MODELS", description: "Show experimental models", category: "Models" },
        EnvVar { name: "OPENCODE_MODELS_URL", description: "Custom models.dev-compatible URL", category: "Models" },
        EnvVar { name: "OPENCODE_SERVER_PASSWORD", description: "Basic auth for serve/web", category: "Server" },
        EnvVar { name: "OPENCODE_SERVER_USERNAME", description: "Auth username (default opencode)", category: "Server" },
        EnvVar { name: "OPENCODE_GIT_BASH_PATH", description: "Custom Git Bash path on Windows", category: "Shell" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL", description: "Umbrella experimental flag", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_PLAN_MODE", description: "Plan mode", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_BACKGROUND_SUBAGENTS", description: "Background subagents", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_SCOUT", description: "Scout subagent", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_FILEWATCHER", description: "File watcher", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_LSP_TOOL", description: "LSP tool", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_MARKDOWN", description: "Markdown improvements", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_PARALLEL", description: "Parallel search", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_WORKSPACES", description: "Workspaces", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_ICON_DISCOVERY", description: "Icon discovery", category: "Experimental" },
        EnvVar { name: "OPENCODE_EXPERIMENTAL_BASH_DEFAULT_TIMEOUT_MS", description: "Bash default timeout", category: "Experimental" },
        EnvVar { name: "OPENCODE_DISABLE_CLAUDE_CODE", description: "Skip .claude directory", category: "Interop" },
        EnvVar { name: "OPENCODE_DISABLE_CLAUDE_CODE_PROMPT", description: "Skip ~/.claude/CLAUDE.md", category: "Interop" },
        EnvVar { name: "OPENCODE_DISABLE_CLAUDE_CODE_SKILLS", description: "Skip .claude/skills", category: "Interop" },
    ]
}

fn subcommands() -> &'static [Subcommand] {
    &[
        Subcommand { cmd: "opencode", desc: "Interactive TUI" },
        Subcommand { cmd: "opencode run", desc: "Non-interactive prompt" },
        Subcommand { cmd: "opencode serve", desc: "Headless server" },
        Subcommand { cmd: "opencode web", desc: "Web UI server" },
        Subcommand { cmd: "opencode attach", desc: "Attach TUI to remote server" },
        Subcommand { cmd: "opencode acp", desc: "Agent Client Protocol bridge" },
        Subcommand { cmd: "opencode auth", desc: "Manage providers and credentials" },
        Subcommand { cmd: "opencode models", desc: "List models" },
        Subcommand { cmd: "opencode mcp", desc: "MCP servers (add/list/auth/logout/debug)" },
        Subcommand { cmd: "opencode agent", desc: "Manage agents (create/list)" },
        Subcommand { cmd: "opencode session", desc: "Sessions (list/delete)" },
        Subcommand { cmd: "opencode export", desc: "Export session JSON" },
        Subcommand { cmd: "opencode import", desc: "Import session" },
        Subcommand { cmd: "opencode stats", desc: "Token usage and cost" },
        Subcommand { cmd: "opencode github", desc: "GitHub agent (install/run)" },
        Subcommand { cmd: "opencode pr", desc: "Fetch PR and run OpenCode" },
        Subcommand { cmd: "opencode plugin", desc: "Install plugin module" },
        Subcommand { cmd: "opencode db path", desc: "Print database path" },
        Subcommand { cmd: "opencode debug", desc: "Debugging utilities" },
        Subcommand { cmd: "opencode upgrade", desc: "Self-update" },
        Subcommand { cmd: "opencode uninstall", desc: "Uninstall" },
        Subcommand { cmd: "opencode completion", desc: "Shell completion script" },
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
                "tools.write", "tools.bash", "tools.edit", "tools.read", "tools.grep", "edit", "bash",
                "compaction.auto", "compaction.prune", "compaction.reserved",
                "theme", "share", "autoupdate", "snapshot", "username", "instructions",
                "mouse", "attention.enabled", "attention.notifications",
                "experimental.plan_mode", "experimental.background_subagents", "experimental.scout",
                "experimental.workspaces", "experimental.hooks", "experimental.lsp_tool", "experimental.parallel",
                "experimental.filewatcher", "lsp", "formatter", "watcher.ignore",
            ],
            values: serde_json::json!({
                "model": "anthropic/claude-sonnet-4-6",
                "small_model": "anthropic/claude-haiku-4-5",
                "enabled_providers": ["anthropic", "openai", "openrouter"],
                "default_agent": "build",
                "subagent_depth": 2,
                "tools.write": true,
                "tools.bash": true,
                "tools.edit": true,
                "tools.read": true,
                "tools.grep": true,
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
                "instructions": ["AGENTS.md", "CONTRIBUTING.md"],
                "mouse": true,
                "attention.enabled": true,
                "attention.notifications": true,
                "experimental.plan_mode": true,
                "experimental.background_subagents": true,
                "experimental.scout": true,
                "experimental.workspaces": true,
                "experimental.hooks": true,
                "experimental.lsp_tool": true,
                "experimental.parallel": true,
                "experimental.filewatcher": true,
                "lsp": true,
                "formatter": true,
                "watcher.ignore": ["node_modules/**", "dist/**", ".git/**"]
            }),
        },
        Preset {
            id: "titanium-max",
            name: "Titanium Max",
            description: "Everything on: deeper subagents, server bind, demo MCP stub, parallel + scout.",
            enabled: &[
                "model", "small_model", "default_agent", "subagent_depth",
                "tools.write", "tools.bash", "edit", "bash",
                "compaction.auto", "server.port", "server.hostname", "server.mdns",
                "experimental.plan_mode", "experimental.background_subagents", "experimental.scout",
                "experimental.workspaces", "experimental.hooks", "experimental.lsp_tool", "experimental.parallel",
                "lsp", "formatter", "mcp.demo.enabled", "mcp.demo.url", "username", "share",
                "attachment.image.auto_resize", "provider.timeout", "mouse", "attention.enabled",
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
                "server.mdns": false,
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
                "share": "disabled",
                "attachment.image.auto_resize": true,
                "provider.timeout": 600000,
                "mouse": true,
                "attention.enabled": true
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

    let mut provider_opts = Map::new();
    for key in ["provider.timeout", "provider.chunkTimeout", "provider.setCacheKey"] {
        if en.contains(key) {
            if let Some(raw) = values.get(key) {
                if let Some(f) = fields().into_iter().find(|f| f.id == key) {
                    if let Some(val) = coerce(&f, raw) {
                        let short = key.strip_prefix("provider.").unwrap();
                        provider_opts.insert(short.to_string(), val);
                    }
                }
            }
        }
    }
    if !provider_opts.is_empty() {
        let mut provider = Map::new();
        let mut def = Map::new();
        def.insert("options".into(), Value::Object(provider_opts));
        provider.insert("default".into(), Value::Object(def));
        root.insert("provider".into(), Value::Object(provider));
    }

    for f in fields() {
        if !en.contains(f.id) {
            continue;
        }
        if f.id.starts_with("provider.") {
            continue;
        }
        if f.id == "mcp.demo.enabled" || f.id == "mcp.demo.type" {
            continue;
        }
        if f.id == "mcp.demo.url" {
            if en.contains("mcp.demo.enabled") && values.get("mcp.demo.enabled").and_then(|v| v.as_bool()) == Some(true) {
                let url = values
                    .get("mcp.demo.url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("https://mcp.example.com/sse");
                let ty = values
                    .get("mcp.demo.type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("remote");
                let mut mcp = Map::new();
                mcp.insert(
                    "demo".into(),
                    json!({ "type": ty, "url": url, "enabled": true }),
                );
                root.insert("mcp".into(), Value::Object(mcp));
            }
            continue;
        }
        if f.id.starts_with("attachment.") || f.id.starts_with("attention.") {
            let Some(raw) = values.get(f.id) else { continue };
            let Some(val) = coerce(&f, raw) else { continue };
            set_path(&mut root, f.path, val);
            continue;
        }
        let Some(raw) = values.get(f.id) else { continue };
        let Some(val) = coerce(&f, raw) else { continue };
        set_path(&mut root, f.path, val);
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let config_out = serde_json::to_string_pretty(&Value::Object(root)).unwrap_or_else(|_| "{}".into());

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
    if en.contains("experimental.filewatcher")
        && values.get("experimental.filewatcher").and_then(|v| v.as_bool()) == Some(true)
    {
        env.push_str("export OPENCODE_EXPERIMENTAL_FILEWATCHER=1\n");
    }
    if en.contains("mouse") && values.get("mouse").and_then(|v| v.as_bool()) == Some(false) {
        env.push_str("export OPENCODE_DISABLE_MOUSE=1\n");
    }
    env.push_str("# export OPENCODE_CONFIG=~/.config/opencode/opencode.json\n");

    let mut cli = format!("# OpenCode Titanium launches\n# Generated: {}\n\n", now);
    cli.push_str("opencode\n");
    cli.push_str("opencode run \"implement the next failing test\"\n");
    cli.push_str("opencode serve\n");
    cli.push_str("opencode web\n");
    cli.push_str("opencode models\n");
    cli.push_str("opencode agent list\n");

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
