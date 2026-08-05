# Agent Config Builders — Titanium

Interactive config builders for **Grok Build**, **Codex**, and **OpenCode**.

Titanium profiles are the sensible defaults. **Grok** default preset is `Titanium · xbgst + livepatch ban` (always-approve, grok-4.5, xbgst-stack, GP/explore toggles off).

## Live

| Product | GitHub Pages | Vercel |
| --- | --- | --- |
| Hub | https://veigapunk.github.io/grok-build-config-builder/ | (this repo → Vercel) |
| Grok Build | https://veigapunk.github.io/grok-build-config-builder/grok/ | https://grok-build-config-builder.vercel.app |
| Codex Titanium | https://veigapunk.github.io/grok-build-config-builder/codex/ | https://codex-titanium-config.vercel.app |
| OpenCode Titanium | https://veigapunk.github.io/grok-build-config-builder/opencode/ | https://opencode-titanium-config.vercel.app |

## Stack

- **Preview / Vercel:** TanStack Start + React 19 + Tailwind v4 (Nitro `vercel` preset)
- **GitHub Pages:** static multi-product hub in `pages-multi/`

## Run

```bash
sh startup.sh
# or
npm run dev   # 0.0.0.0:8080
```

## Layout

```
src/                  # React app (hub + builders)
public/schemas/       # Grok / Codex / OpenCode field schemas
pages-multi/          # Static multi-product site (GitHub Pages source)
```

## License

MIT
