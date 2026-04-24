mod auth;
mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod store;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Json, Router,
};
use config::Config;
use serde_json::json;
use store::memory::MemoryStore;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_scaffold=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let addr = format!("{}:{}", config.host, config.port);

    let state = AppState {
        store: MemoryStore::new(),
        config,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/me", get(handlers::auth::me))
        .route(
            "/users",
            get(handlers::users::list_users).post(handlers::users::create_user),
        )
        .route(
            "/users/:id",
            get(handlers::users::get_user)
                .put(handlers::users::update_user)
                .delete(handlers::users::delete_user),
        )
        .layer(CorsLayer::permissive())
        .layer(from_fn(middleware::logging::session_logger))
        .with_state(state);

    info!("Listening on http://{addr}");
    let listener = TcpListener::bind(&addr).await.expect("failed to bind TCP listener");
    axum::serve(listener, app).await.expect("server error");
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
