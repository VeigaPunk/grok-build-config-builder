# Changelog

## 2.0.0 — 2026-08-05

### Titanium Config Builders v2

New major surface for the multi-product agent config hub.

**Highlights**
- Polished multi-product hub (Grok · Codex · OpenCode)
- Grok default preset: **Titanium · xbgst + livepatch ban**
- Full interactive builders: toggles, presets, search, Preview / Reference
- Download / copy for config patches, env exports, CLI launch lines, markdown docs
- React (TanStack Start) app for Vercel + static `pages-multi/` for GitHub Pages
- Schemas wired for all three products + recommended `config.toml`

**Grok xbgst defaults**
- `grok-4.5` + always-approve
- xbgst-stack + marketplace sources (xAI / grok-marketplace / ds4cc)
- Hard-ban general-purpose / explore (livepatch-aligned)
- Flat subagent tree (`max_depth = 1`)

**Deploy**
- GitHub Pages: https://veigapunk.github.io/grok-build-config-builder/
- Vercel-ready Nitro preset (`npm run build` → `.vercel/output`)
