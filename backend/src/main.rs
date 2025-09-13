// backend/src/main.rs

use axum::{routing::get, Router};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;            // ✅ tambah ini
// use std::env; // ❌ tidak dipakai, hapus

mod util;
mod db;
mod routes;
mod models;
mod security;
mod middleware;
mod cosm;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,sqlx::query=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = util::config::Config::from_env()?;

    let pool = db::mysql::create_pool(&cfg.mysql_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;   // ✅ tetap, karena migrations sudah dicopy

    let redis = db::redis::create_client(&cfg.redis_url)?;

    let state = AppState {
        pool,
        redis,
        jwt_secret: cfg.jwt_secret.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(routes::public::router())
        .merge(routes::auth::router())
        .merge(routes::admin::router())
        .with_state(state)
        .layer(cors);

    let addr = format!("0.0.0.0:{}", cfg.port);
    let listener = TcpListener::bind(&addr).await?;     // ✅ bind pakai tokio
    tracing::info!("listening on http://{}", addr);

    axum::serve(listener, app).await?;                  // ✅ axum 0.7 style
    Ok(())
}
