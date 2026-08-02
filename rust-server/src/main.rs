//! Multi-product config builders: Grok · Codex Titanium · OpenCode Titanium

mod generate;
mod products;
mod schema;

use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use products::{generate_for, schema_value, ProductId};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{
    compression::CompressionLayer, set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "grok-config-server")]
struct Args {
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    host: String,
    #[arg(long, env = "PORT", default_value_t = 8080)]
    port: u16,
    #[arg(long, env = "STATIC_DIR", default_value = "/workspace/rust-server/static")]
    static_dir: PathBuf,
}

#[derive(Clone)]
struct AppState {
    static_dir: PathBuf,
    hub: Arc<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenReq {
    enabled: Vec<String>,
    values: BTreeMap<String, Value>,
}

async fn health() -> &'static str { "ok" }

async fn hub(State(st): State<AppState>) -> Html<String> {
    Html(st.hub.as_ref().clone())
}

async fn product_page(State(st): State<AppState>, Path(product): Path<String>) -> Response {
    if ProductId::parse(&product).is_none() {
        return (StatusCode::NOT_FOUND, "unknown product").into_response();
    }
    let path = st.static_dir.join("index.html");
    match std::fs::read_to_string(&path) {
        Ok(mut html) => {
            // inject product slug
            html = html.replace("data-product=\"grok\"", &format!("data-product=\"{}\"", product));
            if !html.contains("data-product=") {
                html = html.replace("<body>", &format!("<body data-product=\"{}\">", product));
            }
            Html(html).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn styles(State(st): State<AppState>) -> Response {
    serve_static(&st.static_dir.join("styles.css"), "text/css; charset=utf-8")
}
async fn app_js(State(st): State<AppState>) -> Response {
    serve_static(&st.static_dir.join("app.js"), "application/javascript; charset=utf-8")
}

fn serve_static(path: &std::path::Path, ct: &'static str) -> Response {
    match std::fs::read_to_string(path) {
        Ok(body) => (StatusCode::OK, [(header::CONTENT_TYPE, ct)], body).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn api_schema(Path(product): Path<String>) -> Response {
    let Some(id) = ProductId::parse(&product) else {
        return (StatusCode::NOT_FOUND, "unknown product").into_response();
    };
    let json = serde_json::to_string(&schema_value(id)).unwrap();
    (StatusCode::OK, [(header::CONTENT_TYPE, "application/json; charset=utf-8")], json).into_response()
}

async fn api_generate(Path(product): Path<String>, Json(req): Json<GenReq>) -> Response {
    let Some(id) = ProductId::parse(&product) else {
        return (StatusCode::NOT_FOUND, "unknown product").into_response();
    };
    let out = generate_for(id, &req.enabled, &req.values);
    Json(out).into_response()
}

fn hub_html() -> String {
    r#"<!doctype html>
<html lang="en"><head>
<meta charset="UTF-8"/><meta name="viewport" content="width=device-width,initial-scale=1"/>
<title>Agent Config Builders — Titanium</title>
<link rel="stylesheet" href="/static/styles.css"/>
</head>
<body>
<main class="wrap" style="padding:48px 24px;max-width:56rem">
  <div class="eyebrow">Rust · multi-product</div>
  <h1 style="margin-top:8px">Config builders</h1>
  <p class="lede">Toggle every setting. Download patches. Titanium profiles are the sensible defaults.</p>
  <div style="display:grid;gap:16px;margin-top:32px;grid-template-columns:repeat(auto-fill,minmax(240px,1fr))">
    <a class="panel" href="/grok/" style="padding:20px;text-decoration:none;color:inherit">
      <strong style="font-size:16px">Grok Build</strong>
      <p style="margin:8px 0 0;color:var(--fg-muted);font-size:13px">~/.grok/config.toml · Grok Build CLI</p>
    </a>
    <a class="panel" href="/codex/" style="padding:20px;text-decoration:none;color:inherit;border-color:color-mix(in oklab,#c8ccd4 35%,transparent)">
      <strong style="font-size:16px">Codex Titanium</strong>
      <p style="margin:8px 0 0;color:var(--fg-muted);font-size:13px">~/.codex/config.toml · Titanium default profile</p>
    </a>
    <a class="panel" href="/opencode/" style="padding:20px;text-decoration:none;color:inherit;border-color:color-mix(in oklab,#c8ccd4 35%,transparent)">
      <strong style="font-size:16px">OpenCode Titanium</strong>
      <p style="margin:8px 0 0;color:var(--fg-muted);font-size:13px">opencode.json · wild Titanium build</p>
    </a>
  </div>
</main>
</body></html>"#.into()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let args = Args::parse();
    let state = AppState {
        static_dir: args.static_dir.clone(),
        hub: Arc::new(hub_html()),
    };
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/", get(hub))
        .route("/static/styles.css", get(styles))
        .route("/static/app.js", get(app_js))
        .route("/{product}/", get(product_page))
        .route("/{product}", get(product_page))
        .route("/api/{product}/schema", get(api_schema))
        .route("/api/{product}/generate", post(api_generate))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse().unwrap();
    tracing::info!(%addr, "multi-product config builders listening");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
