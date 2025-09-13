use crate::AppState;
use axum::{http::{Request, StatusCode}, response::IntoResponse};
use axum::body::Body;

pub async fn guard<B>(state: AppState, req: Request<B>, next: axum::middleware::Next<B>) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    if !path.starts_with("/admin") {
        return next.run(req).await;
    }
    let Some(auth) = req.headers().get(axum::http::header::AUTHORIZATION) else {
        return (StatusCode::UNAUTHORIZED, "missing Authorization").into_response();
    };
    let s = auth.to_str().unwrap_or("");
    let token = s.strip_prefix("Bearer ").unwrap_or("");
    match crate::security::jwt::validate(token, &state.jwt_secret) {
        Ok(_) => next.run(req).await,
        Err(_) => (StatusCode::UNAUTHORIZED, "invalid token").into_response(),
    }
}
