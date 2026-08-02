# Grok Build Config Builder

Interactive config reference and patch builder for the [Grok Build CLI](https://x.ai/cli).

- Toggle every `config.toml` / env / CLI setting
- Download a custom patch for `~/.grok/config.toml`
- Export env vars and launch snippets
- Full markdown reference included

## Live

- **Vercel:** (production deploy — see Actions / Vercel dashboard)
- **GitHub Pages:** https://veigapunk.github.io/grok-build-config-builder/

## Local development

```bash
npm install
npm run dev      # http://localhost:8080
npm run build
npm run typecheck
```

## Stack

React 19 · TypeScript · Vite · TanStack Start · Tailwind v4

## Docs

See [`public/docs/grok-build-cli-config-reference.md`](./public/docs/grok-build-cli-config-reference.md) or download from the app UI.

## License

MIT
