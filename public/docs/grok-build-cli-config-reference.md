# Grok Build CLI — Complete Config Reference

> Compiled reference for **Grok Build** (`grok`) configuration: CLI flags, `config.toml`, environment variables, layers, sandbox, and permissions.
>
> Latest documented binary track: **v0.2.117** (Jul 30, 2026) · Powered by **Grok 4.5**
>
> Sources: [x.ai/cli](https://x.ai/cli) · [Changelog](https://x.ai/build/changelog) · [CLI reference](https://docs.x.ai/build/cli/reference) · [Settings](https://docs.x.ai/build/settings) · [Settings reference](https://docs.x.ai/build/settings/reference) · [Enterprise](https://docs.x.ai/build/enterprise)

---

## Install & version

```bash
# macOS / Linux
curl -fsSL https://x.ai/cli/install.sh | bash

# Windows PowerShell
irm https://x.ai/cli/install.ps1 | iex

grok version
grok update --check
grok update --stable
```

Requires **SuperGrok** or **X Premium+**. Sign in with `grok login` (or `grok login --device-auth` for headless/SSH).

Open source harness: [github.com/xai-org/grok-build](https://github.com/xai-org/grok-build) (Apache 2.0).

---

## Configuration layers

Grok loads configuration from five layers (lowest → highest priority):

| Priority | Source | Purpose |
| ---: | --- | --- |
| 1 | `/etc/grok/managed_config.toml` | System-wide managed defaults |
| 2 | `~/.grok/managed_config.toml` | Per-user managed defaults |
| 3 | `~/.grok/config.toml` (`$GROK_HOME/config.toml`) | User preferences |
| 4 | `~/.grok/requirements.toml` | User-level pinned settings |
| 5 | `/etc/grok/requirements.toml` | System pins (highest, fail-closed) |

**Project** `.grok/config.toml` may only contribute MCP servers, plugins, and permission rules — not full user UI prefs.

All layers support `[[version_overrides]]` and `$VAR` expansion. Values in `requirements.toml` cannot be overridden by lower layers, remote settings, or user config.

```bash
grok inspect
grok inspect --json
```

---

## Subcommands

| Command | Description |
| --- | --- |
| `grok login [--device-auth]` | Sign in (device-code for headless) |
| `grok logout` | Sign out and clear credentials |
| `grok inspect [--json]` | Show discovered config for current directory |
| `grok models` | List available models |
| `grok mcp <list\|add\|remove\|doctor>` | Manage MCP servers |
| `grok plugin <…>` | Install/enable/disable plugins |
| `grok plugin marketplace <…>` | Manage marketplace sources |
| `grok sessions <list\|search\|delete>` | Manage sessions |
| `grok export <id> [out]` | Export session as Markdown |
| `grok import [targets…]` | Import sessions from Claude Code |
| `grok memory clear [--workspace\|--global\|--all]` | Clear memory files |
| `grok worktree <list\|show\|rm\|gc>` | Manage session worktrees |
| `grok dashboard` | Open Agent Dashboard |
| `grok agent stdio` | Run as ACP agent over stdin/stdout |
| `grok wrap <command…>` | PTY with OSC 52 clipboard forwarding |
| `grok update [--check\|--version\|--alpha\|--stable]` | Update CLI |
| `grok version` | Print version |
| `grok completions <shell>` | Shell completion scripts |
| `grok setup` | Fetch and install managed configuration |

Running `grok` with no arguments starts the interactive TUI.

---

## Session launch flags

| Flag | Category | Description |
| --- | --- | --- |
| `--cwd <PATH>` | Session | Set working directory |
| `-r, --resume [<ID>]` | Session | Resume session (omit ID = most recent) |
| `-c, --continue` | Session | Continue most recent session for this directory |
| `-s, --session-id <UUID>` | Session | Use a specific UUID for a new session |
| `--fork-session` | Session | When resuming, fork into a new session ID |
| `-w, --worktree [<NAME>]` | Session | Start session in a new git worktree |
| `--ref <REF>` | Session | Branch, tag, or commit for the worktree |
| `-m, --model <MODEL>` | Model | Model ID |
| `--effort <LEVEL>` | Model | Reasoning effort level |
| `--always-approve / --yolo` | Permissions | Auto-approve all tool executions |
| `--allow <RULE>` | Permissions | Permission allow rule (e.g. 'Bash(git *)') |
| `--deny <RULE>` | Permissions | Permission deny rule |
| `--sandbox <PROFILE>` | Permissions | Sandbox profile: off \| workspace \| read-only \| strict |
| `--rules <TEXT>` | Prompt | Append extra rules to system prompt |
| `--system-prompt-override <TEXT>` | Prompt | Replace system prompt entirely |
| `--tools <LIST>` | Tools | Allow only these built-in tools |
| `--disallowed-tools <LIST>` | Tools | Disallow built-in tools |
| `--max-turns <N>` | Agent | Maximum agent turns |
| `--no-plan` | Agent | Disable planning |
| `--no-subagents` | Agent | Disable subagents |
| `--no-memory` | Agent | Disable memory |
| `--disable-web-search` | Agent | Disable web search |
| `--experimental-memory` | Agent | Enable cross-session memory |
| `--oauth` | Auth | Use OAuth when welcome screen starts auth |
| `-p, --single <PROMPT>` | Headless | Headless one-shot prompt |
| `--output-format <FMT>` | Headless | plain \| json \| streaming-json |
| `--no-alt-screen` | Headless | Inline (no fullscreen TUI takeover) |
| `--no-auto-update` | Headless | Skip background update checks |

### Claude Code aliases

| Alias | Maps to |
| --- | --- |
| `--allowedTools` | `--tools` |
| `--disallowedTools` | `--disallowed-tools` |
| `--append-system-prompt` | `--rules` |
| `--system-prompt` | `--system-prompt-override` |
| `--dangerously-skip-permissions` | `--always-approve` |

---

## Sandbox profiles

| Profile | Read | Write | Network (Linux) | Use |
| --- | --- | --- | --- | --- |
| `off` | unrestricted | unrestricted | allowed | Default |
| `workspace` | everywhere | CWD + `~/.grok/temp` | allowed | Normal development |
| `devbox` | everywhere | top-level dirs except `/data` | allowed | Cloud devbox |
| `read-only` | everywhere | `~/.grok/temp` | blocked | Code review |
| `strict` | CWD/system | CWD + temp | blocked | Untrusted repos |

Enable via CLI `--sandbox workspace`, config `[sandbox] profile = "workspace"`, or env `GROK_SANDBOX=workspace`.

Custom profiles live in `~/.grok/sandbox.toml` or `.grok/sandbox.toml`:

```toml
[profiles.my-profile]
extends = "workspace"
restrict_network = true
deny = ["/secrets", "**/.env", "**/*.pem"]
```

---

## Permissions

- `permission_mode`: `auto` (default) | `ask` | `always-approve`
- Legacy keys: `approval_mode`, `yolo = true`

```toml
rules = [
  { action = "allow", tool = "bash", pattern = "git *" },
  { action = "allow", tool = "read" },
  { action = "deny",  tool = "bash", pattern = "rm -rf *" },
]
```

CLI: `--allow '<pattern>'` / `--deny '<pattern>'`. Tools: `Bash`, `Edit`, `Read`, `Grep`, `MCPTool`, `WebFetch`, `WebSearch`. **Deny always overrides allow.**

---

## config.toml — starter sample

```toml
[models]
default = "grok-build"
web_search = "grok-4.5"

[model."grok-4.5"]
model = "grok-4.5"
base_url = "https://api.x.ai/v1"
name = "Grok 4.5"
env_key = "XAI_API_KEY"
api_backend = "responses"
temperature = 0.7
top_p = 0.95
max_completion_tokens = 8192
context_window = 1000000
supports_backend_search = true

[sandbox]
profile = "workspace"
auto_allow_bash = false

[cli]
auto_update = true
channel = "stable"

[ui]
theme = "auto"
show_thinking_blocks = true
screen_mode = "fullscreen"

[harness]
disable_codebase_upload = true

[telemetry]
trace_upload = false

[features]
telemetry = false

[mcp_servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/allowed"]
enabled = true
startup_timeout_sec = 30
tool_timeout_sec = 6000
```

### Models section keys

| Key | Type | Default | Env / CLI | Description |
| --- | --- | --- | --- | --- |
| `models.default` | string | `grok-build` | `GROK_DEFAULT_MODEL` / `-m` | Default session model |
| `models.web_search` | string | — | `GROK_WEB_SEARCH_MODEL` | web_search tool model |
| `models.default_reasoning_effort` | enum | — | `--effort` | Default reasoning effort |
| `models.session_summary` | string | — | — | Session summary model |
| `models.image_description` | string | — | — | Image description model |
| `models.temperature` | number | — | — | Global temperature |
| `models.top_p` | number | — | — | Global top_p |
| `models.max_completion_tokens` | number | — | — | Global max tokens |
| `models.max_retries` | number | — | — | Inference retries |
| `models.stream_tool_calls` | boolean | — | — | Tool-call streaming shape |
| `models.allowed_models` | list | — | — | Restrict picker / -m |
| `models.hidden_models` | list | — | — | Hide from picker |
| `models.disabled_models` | list | — | — | Remove from catalog |

### Per-model `[model."<id>"]`

| Key | Description |
| --- | --- |
| `model` | API model id |
| `base_url` | Provider endpoint |
| `name` / `description` | Picker labels |
| `api_key` / `env_key` | Auth (prefer env_key) |
| `api_backend` | `chat_completions` \| `responses` \| `messages` |
| `temperature` / `top_p` / `max_completion_tokens` | Sampling |
| `context_window` | Tokens; drives auto-compact |
| `supports_backend_search` | Server-side search tools |
| `supports_reasoning_effort` / `reasoning_effort` | Reasoning controls |
| `stream_tool_calls` / `max_retries` / `inference_idle_timeout_secs` | Reliability |

### Sandbox, session, CLI, UI, privacy

| Key | Default | Description |
| --- | --- | --- |
| `sandbox.profile` | `off` | `off` \| `workspace` \| `read-only` \| `strict` \| custom |
| `sandbox.auto_allow_bash` | `false` | Skip bash prompts when sandboxed |
| `session.auto_compact_threshold_percent` | `85` | Auto-compact threshold |
| `cli.auto_update` | `true` | Check updates on launch |
| `cli.channel` | — | `stable` \| `alpha` |
| `cli.show_tips` | — | Startup tips |
| `hints.new_session_worktree_mode` | `never` | `/new` worktree: ask \| always \| never |
| `hints.fork_worktree_mode` | `ask` | `/fork` worktree |
| `ui.compact_mode` | `false` | Denser padding |
| `ui.theme` | — | Color theme |
| `ui.screen_mode` | `fullscreen` | fullscreen \| minimal |
| `ui.show_thinking_blocks` | `true` | Show reasoning blocks |
| `ui.show_timestamps` | `true` | Clock next to messages |
| `ui.show_timeline` | `false` | Tick rail vs scrollbar |
| `ui.page_flip_on_send` | `true` | Snap prompt to top |
| `ui.disable_bypass_permissions_mode` | — | Enterprise: block --yolo |
| `harness.disable_codebase_upload` | — | Skip whole-repo upload request |
| `telemetry.trace_upload` | — | Upload telemetry traces |
| `features.telemetry` | — | Master telemetry feature flag |
| `plugins.paths` / `enabled` / `disabled` | — | Plugin discovery control |
| `auth.oidc.client_id` | — | Enterprise OIDC |
| `auth.auth_provider_command` | — | External token command |
| `grok_com_config.disable_api_key_auth` | — | Force IdP login |
| `grok_com_config.force_login_team_uuid` | — | Restrict team login |

---

## Environment variables

| Variable | Category | Description |
| --- | --- | --- |
| `GROK_HOME` | Paths | Home for config/auth/sessions (default `~/.grok`) |
| `XAI_API_KEY` | Auth | API key for headless / CI |
| `GROK_DEFAULT_MODEL` | Models | Session default model |
| `GROK_WEB_SEARCH_MODEL` | Models | web_search model |
| `GROK_MODELS_BASE_URL` | Models | Custom inference base URL |
| `GROK_MODELS_LIST_URL` | Models | Override model list URL |
| `GROK_XAI_API_BASE_URL` | Models | xAI API base (default `https://api.x.ai/v1`) |
| `GROK_DISABLE_AUTOUPDATER` | CLI | Suppress auto-updater when set |
| `GROK_SANDBOX` | Sandbox | Sandbox profile name |
| `GROK_SANDBOX_AUTO_ALLOW_BASH` | Sandbox | 1/0 auto-allow bash |
| `GROK_RESPECT_GITIGNORE` | Tools | 1/0 force gitignore filtering |
| `GROK_WEB_FETCH` | Tools | 1/0 enable web_fetch |
| `GROK_WEB_FETCH_PROXY` | Tools | Egress proxy for web_fetch |
| `GROK_MEMORY` | Features | 1/0 cross-session memory |
| `GROK_SUBAGENTS` | Features | 1/0 subagents |
| `GROK_SUBAGENTS_MAX_DEPTH` | Features | Max nesting depth (default 1 = flat tree); TOML: `[subagents] max_depth` |
| `GROK_AGENT` | Features | Agent name or path |
| `GROK_WRITE_FILE` | Tools | 0 disables write tool |
| `GROK_TOOL_SEARCH` | Tools | 1/0 MCP tool discovery |
| `GROK_LSP_TOOLS` | Tools | 1/0 LSP code-intel |
| `GROK_THEME` | UI | Color theme |
| `GROK_SHOW_THINKING_BLOCKS` | UI | 1/0 reasoning blocks |
| `GROK_GROUP_TOOL_VERBS` | UI | 1/0 fold tool rows |
| `GROK_COLLAPSED_EDIT_BLOCKS` | UI | 1/0 collapse edits |
| `GROK_PROMPT_SUGGESTIONS` | UI | 1/0 ghost suggestions |
| `GROK_SCROLL_SPEED` | UI | 1–100 |
| `GROK_SCROLL_MODE` | UI | auto \| wheel \| trackpad |
| `GROK_SCROLL_LINES` | UI | Lines per tick 1–10 |
| `GROK_INVERT_SCROLL` | UI | 1/0 invert scroll |
| `GROK_DEFAULT_SELECTED_PERMISSION` | Permissions | First prompt default |
| `GROK_REMEMBER_TOOL_APPROVALS` | Permissions | 1/0 always-allow options |
| `GROK_MOUSE_REPORTING_TOGGLE` | UI | 1/0 Ctrl+R mouse toggle |
| `GROK_MCP_STARTUP_TIMEOUT_SECS` | MCP | Handshake timeout (default 30) |
| `MCP_TIMEOUT` | MCP | Claude-compatible timeout (ms) |
| `GROK_LOG_FILE` | Logging | Log file path |
| `RUST_LOG` | Logging | Log filter e.g. `debug` |
| `GROK_CRASH_HANDLER` | Logging | 1/0 crash reports under `$GROK_HOME/crash` |
| `HTTPS_PROXY` / `HTTP_PROXY` / `NO_PROXY` | Network | Standard proxies |
| `GROK_OIDC_ISSUER` | Auth | Enterprise OIDC issuer |
| `GROK_OIDC_CLIENT_ID` | Auth | Enterprise OIDC client |
| `GROK_POOL_IDLE_TIMEOUT_SECS` | Network | HTTP pool idle (default 90) |
| `GROK_EXTRA_CA_BUNDLE` | Network | Extra TLS roots (v0.2.117+) |
| `GROK_CURSOR_*_ENABLED` | Compat | Scan Cursor skills/rules/agents/mcps/hooks |
| `GROK_CLAUDE_*_ENABLED` | Compat | Scan Claude skills/rules/agents/mcps/hooks |

---

## Privacy notes

- Client config keys: `[harness] disable_codebase_upload`, `[telemetry] trace_upload`, `[features] telemetry`.
- xAI also applies **server-side** controls for codebase upload; keep client flags set anyway.
- Use `/privacy` in the TUI for account privacy options.
- Team **Zero Data Retention (ZDR)** is an enterprise control.

---

## Headless examples

```bash
grok -p "summarize this repo" --output-format json --always-approve
grok -p "fix types" --output-format streaming-json --sandbox workspace --no-auto-update
```

---

## Useful links

- Product: https://x.ai/cli
- Changelog: https://x.ai/build/changelog
- CLI reference: https://docs.x.ai/build/cli/reference
- Settings: https://docs.x.ai/build/settings
- Settings reference: https://docs.x.ai/build/settings/reference
- Headless: https://docs.x.ai/build/cli/headless-scripting
- Enterprise: https://docs.x.ai/build/enterprise
- Source: https://github.com/xai-org/grok-build
