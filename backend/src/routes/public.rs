
/*
backend/src/routes/public.rs
*/

use axum::{Router, routing::get, extract::State, Json};
use sqlx::Row;
use uuid::Uuid;
use crate::{AppState, util::error::ApiError, models::poll::PollDto};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/public/polls", get(list_published))
}

async fn list_published(State(st): State<AppState>) -> Result<Json<Vec<PollDto>>, ApiError> {
    let rows = sqlx::query(
        "SELECT id, slug, title, description, options_json, start, end, vote_price, status
         FROM polls WHERE status='published' ORDER BY end DESC")
        .fetch_all(&st.pool).await?;
    let mut out = vec![];
    for r in rows {
        let options: String = r.get("options_json");
        out.push(PollDto{
    id: r.get::<Uuid,_>("id"),
    slug: r.get("slug"),
    title: r.get("title"),
    description: r.get("description"),
    options: serde_json::from_str(&options).unwrap_or_default(),
    start: r.get::<Option<i64>,_>("start"),
    end: r.get::<i64,_>("end"),
    vote_price: r.get("vote_price"),
    status: r.get("status"),
    // NEW:
    onchain_status: "none".to_string(),
    onchain_tx_hash: None,
    onchain_at: None,
});
    }
    Ok(Json(out))
}
