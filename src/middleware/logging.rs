use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use chrono::Utc;
use std::time::Instant;
use tracing::info;

pub async fn session_logger(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let is_authenticated = req.headers().contains_key(axum::http::header::AUTHORIZATION);
    let timestamp = Utc::now();
    let start = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status = response.status().as_u16();

    info!(
        timestamp = %timestamp.format("%Y-%m-%dT%H:%M:%S%.3fZ"),
        method = %method,
        path = %path,
        status = status,
        duration_ms = elapsed.as_millis() as u64,
        authenticated = is_authenticated,
        "request"
    );

    response
}
