// backend/src/middleware/mod.rs
use axum::{http::Request, middleware::Next, response::Response};
use axum::body::Body;

pub async fn example_middleware(req: Request<Body>, next: Next) -> Response {
    tracing::debug!("Middleware: incoming {}", req.uri());
    next.run(req).await
}
