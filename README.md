# Agent Config Builders — Titanium

Interactive config builders for **Grok Build**, **Codex**, and **OpenCode**.  
**Stack: pure Rust** (Axum). Browser QA via **agent-browser** (not Playwright).

Titanium profiles are the sensible defaults.

Schemas are exhaustive against each CLI's official docs: config keys, flags, env vars, and subcommands for Grok Build, Codex, and OpenCode.

## Live

| Product | GitHub Pages | Vercel |
| --- | --- | --- |
| Hub | https://veigapunk.github.io/grok-build-config-builder/ | — |
| Grok Build | https://veigapunk.github.io/grok-build-config-builder/grok/ | https://grok-build-config-builder.vercel.app |
| Codex Titanium | https://veigapunk.github.io/grok-build-config-builder/codex/ | https://codex-titanium-config.vercel.app |
| OpenCode Titanium | https://veigapunk.github.io/grok-build-config-builder/opencode/ | https://opencode-titanium-config.vercel.app |

## Run (Rust)

```bash
sh startup.sh
# multi-product hub on the live preview
```

## Browser smoke (agent-browser)

```bash
npm i -g agent-browser && agent-browser install
node scripts/browser-smoke.mjs http://127.0.0.1:8080/
node scripts/browser-smoke.mjs http://127.0.0.1:8080/codex/
```

## Layout

```
rust-server/          # Axum multi-product API + SPA
pages-multi/          # Static multi-product site (GitHub Pages source)
vercel-product-*/     # Per-product Vercel static packages
scripts/browser-smoke.mjs  # agent-browser smoke (default)
```

## License

MIT
