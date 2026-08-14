# Agent Config Builders — Titanium **v2.0.2**

Interactive config builders for **Grok Build**, **Codex Titanium**, and **OpenCode Titanium**.

> **Pinned release:** [v2.0.0](https://github.com/VeigaPunk/grok-build-config-builder/releases/tag/v2.0.0) · catalog **2.0.2** (Grok Build 1.0.3 · workflows killed)

Titanium profiles are the sensible defaults. **Grok** ships `Titanium · xbgst + livepatch ban` (always-approve, **grok-4.5**, xbgst-stack, GP/explore toggles off, **workflows killed**).

Grok 4.6 is in the catalog as an **unverified** opt-in (`Titanium · grok-4.6 (unverified)`). Titanium stays on 4.5 until the host is tested against 4.6. Workflows are not supported — generated patches always emit `[workflows] enabled = false`.


## Live

| Product | GitHub Pages | Vercel |
| --- | --- | --- |
| **Hub** | https://veigapunk.github.io/grok-build-config-builder/ | this repo |
| Grok Build | https://veigapunk.github.io/grok-build-config-builder/grok/ | https://grok-build-config-builder.vercel.app |
| Codex Titanium | https://veigapunk.github.io/grok-build-config-builder/codex/ | https://codex-titanium-config.vercel.app |
| OpenCode Titanium | https://veigapunk.github.io/grok-build-config-builder/opencode/ | https://opencode-titanium-config.vercel.app |

## Stack

- **App (Vercel):** TanStack Start + React 19 + Tailwind v4 · Nitro `vercel` preset
- **GitHub Pages:** static multi-product hub in `pages-multi/`
- **Schemas:** pure field catalogs for Grok / Codex / OpenCode under `public/schemas/`

## Run

```bash
sh startup.sh
# or
npm run dev   # 0.0.0.0:8080
```

```bash
npm run build      # Vercel output in .vercel/output
npm run typecheck
```

## Layout

```
src/                  # React hub + product builders
public/schemas/       # Grok / Codex / OpenCode schemas
pages-multi/          # Static multi-product site (GitHub Pages)
CHANGELOG.md          # Version history
```

## License

MIT
