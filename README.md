# Grok Build Config Builder

Interactive config reference and patch builder for the [Grok Build CLI](https://x.ai/cli).

**Stack: pure Rust** (Axum). No Node runtime, no Python.

- Toggle every `config.toml` / env / CLI setting  
- Download a custom patch for `~/.grok/config.toml`  
- Export env vars and launch snippets  
- Full markdown reference generated from the Rust schema  

## Toolchain

| Tool | Notes |
| --- | --- |
| **Rust** | stable (`cargo` / `rustc`) |
| **Server** | Axum binary `grok-config-server` |

## Run

```bash
cargo build --manifest-path rust-server/Cargo.toml --release
./rust-server/target/release/grok-config-server --host 0.0.0.0 --port 8080
# or
sh startup.sh
```

Open the live preview. Endpoints:

- `GET /` — SPA  
- `GET /api/schema` — JSON schema  
- `POST /api/generate` — `{ enabled, values }` → toml / env / cli / markdown  
- `GET /healthz` — health  

## Layout

```
rust-server/
  Cargo.toml
  src/
    main.rs       # Axum routes
    schema.rs     # fields, presets, flags, env
    generate.rs   # TOML / env / CLI / markdown
  static/
    index.html
    styles.css
    app.js        # UI (schema loaded from Rust API)
```

## Repository

**https://github.com/VeigaPunk/grok-build-config-builder**

## License

MIT
