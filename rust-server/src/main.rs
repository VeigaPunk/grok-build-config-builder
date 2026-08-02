//! Grok Build Config Builder — pure Rust (Axum).
//! Serves the SPA and generates config.toml / env / CLI patches.

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
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    compression::CompressionLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "grok-config-server", about = "Grok Build config builder (Rust)")]
struct Args {
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    host: String,

    #[arg(long, env = "PORT", default_value_t = 8080)]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    schema_json: Arc<String>,
    index_html: &'static str,
    styles_css: &'static str,
    app_js: &'static str,
}

async fn health() -> &'static str {
    "ok"
}

async fn index(State(state): State<AppState>) -> Html<&'static str> {
    Html(state.index_html)
}

async fn styles(State(state): State<AppState>) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        state.styles_css,
    )
        .into_response()
}

async fn app_js(State(state): State<AppState>) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        state.app_js,
    )
        .into_response()
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
        index_html: include_str!("../static/index.html"),
        styles_css: include_str!("../static/styles.css"),
        app_js: include_str!("../static/app.js"),
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

    tracing::info!(%addr, "Grok Build config builder (Rust) listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("server");
}
