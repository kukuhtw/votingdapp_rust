/*
backend/src/routes/auth.rs
*/

use axum::{routing::post, Router, Json};
use axum::extract::State;
use sqlx::Row;
use uuid::Uuid;

// ⬇⬇⬇ penting: ini yang hilang
use crate::{
    AppState,
    models::admin_user::{RegisterReq, LoginReq, LoginRes},
    util::error::ApiError,
};


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

pub async fn register(
    State(st): State<AppState>,
    Json(req): Json<RegisterReq>
) -> Result<Json<String>, ApiError> {
    let hash = crate::security::password::hash_password(&req.password)?;
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO admins (id, email, password_hash) VALUES (?, ?, ?)")
        .bind(id)
        .bind(&req.email)
        .bind(&hash)
        .execute(&st.pool).await?;
    Ok(Json("ok".to_string()))
}

pub async fn login(
    State(st): State<AppState>,
    Json(req): Json<LoginReq>
) -> Result<Json<LoginRes>, ApiError> {
    let row = sqlx::query("SELECT id, password_hash FROM admins WHERE email = ?")
        .bind(&req.email)
        .fetch_optional(&st.pool).await?;

    let Some(row) = row else {
        return Err(ApiError(axum::http::StatusCode::UNAUTHORIZED, "invalid credentials".into()));
    };
    let id: Uuid = row.get::<Uuid, _>("id");
    let hash: String = row.get("password_hash");

    let ok = crate::security::password::verify_password(&hash, &req.password)?;
    if !ok {
        return Err(ApiError(axum::http::StatusCode::UNAUTHORIZED, "invalid credentials".into()));
    }
    let token = crate::security::jwt::issue(id.to_string(), &st.jwt_secret, 60 * 24)?;
    Ok(Json(LoginRes { token }))
}
