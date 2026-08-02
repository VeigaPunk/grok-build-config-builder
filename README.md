# Agent Config Builders — Titanium

Interactive config builders for **Grok Build**, **Codex**, and **OpenCode**.  
**Stack: pure Rust** (Axum). No Python.

Titanium profiles are the sensible defaults.

## Live

| Product | URL |
| --- | --- |
| **Hub (preview)** | live preview in this workspace |
| **Grok Build** | https://grok-build-config-builder.vercel.app |
| **Codex Titanium** | https://codex-titanium-config.vercel.app |
| **OpenCode Titanium** | https://opencode-titanium-config.vercel.app |

## What you get

- Toggle every setting · download patches · env exports · launch snippets · markdown reference  
- **Codex** → `~/.codex/config.toml` with `[profiles.titanium]` (gpt-5.4, high effort, workspace-write, multi-agent/hooks/memories)  
- **OpenCode** → `opencode.json` Titanium build (Sonnet/Haiku, plan mode, scout, subagents, bash=ask)  
- **Grok** → Grok Build CLI `config.toml`  
- Font: **JetBrainsMonoNL Nerd Font Mono** everywhere  

## Run (Rust)

```bash
cargo build --manifest-path rust-server/Cargo.toml --release
./rust-server/target/release/grok-config-server --host 0.0.0.0 --port 8080
# or
sh startup.sh
```

### Endpoints

- `GET /` — product hub  
- `GET /codex/` · `GET /opencode/` · `GET /grok/` — builders  
- `GET /api/{product}/schema`  
- `POST /api/{product}/generate` — `{ enabled, values }`  
- `GET /healthz`  

## Layout

```
rust-server/
  src/products/{grok,codex,opencode}.rs
  static/{index.html,app.js,styles.css}
vercel-static/          # CDN schemas + product app.js
vercel-deploy-codex/    # Vercel shell for Codex
vercel-deploy-opencode/ # Vercel shell for OpenCode
```

## Repo

https://github.com/VeigaPunk/grok-build-config-builder

## License

MIT
