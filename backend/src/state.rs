// backend/src/state.rs
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub jwt_secret: String,
}
