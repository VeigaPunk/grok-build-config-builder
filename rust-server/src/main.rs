//! Grok Build Config Builder — pure Rust (Axum).

mod generate;
mod schema;

use axum::{
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use generate::{generate_all, GenerateRequest};
use schema::payload;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{
    compression::CompressionLayer, set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "grok-config-server", about = "Grok Build config builder (Rust)")]
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
    schema_json: Arc<String>,
    static_dir: PathBuf,
}

async fn health() -> &'static str {
    "ok"
}

async fn index(State(state): State<AppState>) -> Response {
    let path = state.static_dir.join("index.html");
    match std::fs::read_to_string(&path) {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "missing index.html — set STATIC_DIR",
        )
            .into_response(),
    }
}

async fn styles(State(state): State<AppState>) -> Response {
    let path = state.static_dir.join("styles.css");
    match std::fs::read_to_string(&path) {
        Ok(css) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
            css,
        )
            .into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn app_js(State(state): State<AppState>) -> Response {
    let path = state.static_dir.join("app.js");
    match std::fs::read_to_string(&path) {
        Ok(js) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
            js,
        )
            .into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn api_schema(State(state): State<AppState>) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        state.schema_json.as_ref().clone(),
    )
        .into_response()
}

async fn api_generate(Json(req): Json<GenerateRequest>) -> impl IntoResponse {
    Json(generate_all(&req))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "grok_config_server=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    let schema_json = serde_json::to_string(&payload()).expect("serialize schema");
    let state = AppState {
        schema_json: Arc::new(schema_json),
        static_dir: args.static_dir.clone(),
    };

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/", get(index))
        .route("/static/styles.css", get(styles))
        .route("/static/app.js", get(app_js))
        .route("/api/schema", get(api_schema))
        .route("/api/generate", post(api_generate))
        .fallback(get(index))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", args.host, args.port)
        .parse()
        .expect("invalid host:port");
    tracing::info!(%addr, static_dir = %args.static_dir.display(), "listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("server");
}
