# Grok Build Config Builder

Interactive config reference and patch builder for the [Grok Build CLI](https://x.ai/cli).

- Toggle every `config.toml` / env / CLI setting  
- Download a custom patch for `~/.grok/config.toml`  
- Export env vars and launch snippets  
- Full markdown reference included  

## Repository

**https://github.com/VeigaPunk/grok-build-config-builder**

## Live deploys

| Host | URL | Notes |
| --- | --- | --- |
| **GitHub Pages** | [veigapunk.github.io/grok-build-config-builder](https://veigapunk.github.io/grok-build-config-builder/) | Static SPA from `gh-pages` branch |
| **Vercel** | one-click import below | Full TanStack Start + Nitro production build |

### Enable GitHub Pages (one click)

The `gh-pages` branch is already published by Actions. Enable hosting:

1. Open **[Settings → Pages](https://github.com/VeigaPunk/grok-build-config-builder/settings/pages)**
2. Under **Build and deployment → Source**, choose **Deploy from a branch**
3. Branch: **`gh-pages`** / folder: **`/` (root)** → **Save**

Site URL: **https://veigapunk.github.io/grok-build-config-builder/**

### Deploy to Vercel (one click)

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https%3A%2F%2Fgithub.com%2FVeigaPunk%2Fgrok-build-config-builder&project-name=grok-build-config-builder&repository-name=grok-build-config-builder)

Or:

1. Open [vercel.com/new](https://vercel.com/new)
2. Import **`VeigaPunk/grok-build-config-builder`**
3. Framework preset: leave default / Other — build command is `npm run build` (Nitro writes `.vercel/output`)
4. Deploy

Optional CI: after the first import, add repo secrets `VERCEL_TOKEN`, `VERCEL_ORG_ID`, `VERCEL_PROJECT_ID` so the **Deploy Vercel** workflow can redeploy on push.

## Local development

```bash
npm install
npm run dev          # http://localhost:8080  (TanStack Start SSR)
npm run build        # production / Vercel output
npm run build:pages  # static SPA for GitHub Pages → dist-pages/
npm run typecheck
```

## Stack

React 19 · TypeScript · Vite · TanStack Start · Tailwind v4 · Nitro (Vercel)

## Docs

See [`public/docs/grok-build-cli-config-reference.md`](./public/docs/grok-build-cli-config-reference.md) or download from the app UI.

## License

MIT
