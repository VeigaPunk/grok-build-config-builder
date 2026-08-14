# Changelog

## 2.0.1 — 2026-08-14

### Grok Build 1.0.3 catalog sync

Aligned the Grok builder with Grok Build **1.0.3** (Aug 12) without flipping the Titanium default model.

**Stays grok-4.5** — host has not been verified on Grok 4.6 yet. Type `grok-4.6` (or pick the unverified preset) to generate a trial patch.

**Catalog**
- Flags: `--restore-code` (1.0.0 remote resume), `--plugin-dir`, Claude Code aliases
- Subcommands: `plugin`, `export`, `import`, `memory`, `worktree`, `dashboard`, `agent stdio`, `wrap`, `completions`, `du`, `trace`
- Env: `GROK_CHANNEL`, `GROK_WORKFLOWS`, `GROK_EXTRA_CA_BUNDLE`, `GROK_AGENT`, model URL overrides
- Fields: `ui.max_thoughts_width` (was referenced by Titanium, missing from schema), `[workflows] enabled`, `[memory] enabled`, `[subagents] enabled`
- New opt-in preset: **Titanium · grok-4.6 (unverified)**

**Not changed**
- Titanium · xbgst + livepatch ban still pins `grok-4.5`
- Other presets still use `grok-build` (official coding/agent sample)

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
