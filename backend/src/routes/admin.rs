/*
backend/src/routes/admin.rs
*/

use axum::{Router, routing::{get, post, patch}, extract::{State, Path}, Json};
use sqlx::Row;
use uuid::Uuid;
use crate::{AppState, util::error::ApiError, models::poll::{CreatePollReq, UpdatePollReq, PollDto}};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/polls", get(list_polls).post(create_poll))
        .route("/admin/polls/:id", get(get_poll).patch(update_poll))
        // NEW: push on-chain
        .route("/admin/polls/:id/push-onchain", post(push_onchain))
}

async fn list_polls(State(st): State<AppState>) -> Result<Json<Vec<PollDto>>, ApiError> {
    let rows = sqlx::query(
    "SELECT id, slug, title, description, options_json, start, `end`, vote_price, status,
            COALESCE(onchain_status,'none') AS onchain_status,
            onchain_tx_hash, onchain_at
     FROM polls ORDER BY created_at DESC"
)
.fetch_all(&st.pool).await?;

    let mut out = Vec::new();
    for r in rows {
        let options: String = r.get("options_json");
        out.push(PollDto {
            id: r.get::<Uuid, _>("id"),
            slug: r.get("slug"),
            title: r.get("title"),
            description: r.get("description"),
            options: serde_json::from_str(&options).unwrap_or_default(),
            start: r.get::<Option<i64>, _>("start"),
            end: r.get::<i64, _>("end"),
            vote_price: r.get("vote_price"),
            status: r.get("status"),
        onchain_status: r.get("onchain_status"),
        onchain_tx_hash: r.get::<Option<String>, _>("onchain_tx_hash"),
        onchain_at: r.get::<Option<i64>, _>("onchain_at"),
        });
    }
    Ok(Json(out))
}


async fn get_poll(
    Path(id): Path<String>,
    State(st): State<AppState>,
) -> Result<Json<PollDto>, ApiError> {
    let uid = Uuid::parse_str(&id)
        .map_err(|_| ApiError(axum::http::StatusCode::BAD_REQUEST, "invalid id".into()))?;
    let row = sqlx::query(
        "SELECT id, slug, title, description, options_json, start, `end`, vote_price, status,
                COALESCE(onchain_status,'none') AS onchain_status,
                onchain_tx_hash, onchain_at
         FROM polls WHERE id = ?"
    )
        .bind(uid)
        .fetch_one(&st.pool).await?;

    let options: String = row.get("options_json");
    Ok(Json(PollDto {
        id: row.get::<Uuid, _>("id"),
        slug: row.get("slug"),
        title: row.get("title"),
        description: row.get("description"),
        options: serde_json::from_str(&options).unwrap_or_default(),
        start: row.get::<Option<i64>, _>("start"),
        end: row.get::<i64, _>("end"),
        vote_price: row.get("vote_price"),
        status: row.get("status"),
        onchain_status: row.get("onchain_status"),
        onchain_tx_hash: row.get::<Option<String>, _>("onchain_tx_hash"),
        onchain_at: row.get::<Option<i64>, _>("onchain_at"),
    }))
}

async fn create_poll(
    State(st): State<AppState>,
    Json(req): Json<CreatePollReq>
) -> Result<Json<String>, ApiError> {
    if req.options.len() < 2 {
        return Err(ApiError(axum::http::StatusCode::BAD_REQUEST, "need at least 2 options".into()));
    }
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO polls (id, slug, title, description, options_json, start, end, vote_price, status, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'draft', UNIX_TIMESTAMP())")
        .bind(id)
        .bind(&req.slug)
        .bind(&req.title)
        .bind(&req.description)
        .bind(serde_json::to_string(&req.options).unwrap())
        .bind(req.start)
        .bind(req.end)
        .bind(&req.vote_price)
        .execute(&st.pool).await?;
    Ok(Json(id.to_string()))
}

async fn update_poll(
    Path(id): Path<String>,
    State(st): State<AppState>,
    Json(req): Json<UpdatePollReq>
) -> Result<Json<String>, ApiError> {
    let uid = Uuid::parse_str(&id).map_err(|_| ApiError(axum::http::StatusCode::BAD_REQUEST, "invalid id".into()))?;
    // Build dynamic SQL sederhana (untuk demo)
    
    let mut query = String::from("UPDATE polls SET ");
    let mut args: Vec<(String, String)> = vec![];

    if let Some(v) = req.slug { args.push(("slug".into(), v)); }
    if let Some(v) = req.title { args.push(("title".into(), v)); }
    if let Some(v) = req.description { args.push(("description".into(), v)); }
    if let Some(v) = req.options { args.push(("options_json".into(), serde_json::to_string(&v).unwrap())); }
    if let Some(v) = req.vote_price { args.push(("vote_price".into(), v)); }
    if let Some(v) = req.status { args.push(("status".into(), v)); }
    // start/end
    let mut has_start = false;
    if let Some(v) = req.start {
        has_start = true;
        // null or value
        if v.is_some() {
            args.push(("start".into(), v.unwrap().to_string()));
        } else {
            // gunakan marker khusus "NULL::<type>" untuk di-handle nanti
            args.push(("start".into(), "NULL".into()));
        }
    }
    if let Some(v) = req.end { args.push(("end".into(), v.to_string())); }

    if args.is_empty() {
        return Ok(Json("no changes".into()));
    }

    // Build SQL
    let mut first = true;
    for (k, _) in &args {
        if !first { query.push_str(", "); }
        first = false;
        if k == "start" {
            // nanti di-bind terpisah
            query.push_str("start = ?");
        } else {
            query.push_str(&format!("{k} = ?"));
        }
    }
    query.push_str(", updated_at = UNIX_TIMESTAMP() WHERE id = ?");

    // Prepare query
    let mut q = sqlx::query(&query);
    for (k, v) in args {
        if k == "start" && v == "NULL" {
            q = q.bind(None::<i64>);
        } else if k == "start" {
            let parsed: i64 = v.parse().unwrap();
            q = q.bind(Some(parsed));
        } else {
            q = q.bind(v);
        }
    }
    q = q.bind(uid);
    q.execute(&st.pool).await?;
    Ok(Json("ok".into()))
}

// NEW: push ke on-chain
async fn push_onchain(
    Path(id): Path<String>,
    State(st): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let uid = Uuid::parse_str(&id)
        .map_err(|_| ApiError(axum::http::StatusCode::BAD_REQUEST, "invalid id".into()))?;

    // ambil data poll + status on-chain
    let row = sqlx::query(
        "SELECT slug, title, description, options_json, start, `end`, vote_price,
                COALESCE(onchain_status,'none') AS onchain_status
         FROM polls WHERE id = ?"
    ).bind(uid).fetch_one(&st.pool).await?;

    let onchain_status: String = row.get("onchain_status");
    if onchain_status == "pending" || onchain_status == "success" {
        return Err(ApiError(axum::http::StatusCode::BAD_REQUEST, "already pushed".into()));
    }

    let slug: String = row.get("slug");
    let title: String = row.get("title");
    let options_json: String = row.get("options_json");
    let options: Vec<String> = serde_json::from_str(&options_json).unwrap_or_default();
    let start = row.get::<Option<i64>, _>("start");
    let end = row.get::<i64, _>("end");
    let vote_price: String = row.get("vote_price");

    // (opsional) ambil alamat kontrak dari ENV
    
    let contract_addr = std::env::var("CONTRACT_ADDR")
    .map_err(|_| ApiError(axum::http::StatusCode::INTERNAL_SERVER_ERROR, "CONTRACT_ADDR not set".into()))?;

    // tandai pending
    sqlx::query("UPDATE polls SET onchain_status='pending', onchain_tx_hash=NULL, onchain_at=UNIX_TIMESTAMP() WHERE id=?")
        .bind(uid).execute(&st.pool).await?;

    // panggil TX (stub)
      let res = crate::cosm::tx::create_poll_onchain(
    &contract_addr,
    &crate::cosm::tx::CreatePollMsg { slug, title, options, start, end, vote_price }
).await;

    match res {
        Ok(txhash) => {
            sqlx::query("UPDATE polls SET onchain_status='success', onchain_tx_hash=?, onchain_at=UNIX_TIMESTAMP() WHERE id=?")
                .bind(txhash.clone())
                .bind(uid)
                .execute(&st.pool).await?;
            Ok(Json(txhash))
        }
        Err(e) => {
            sqlx::query("UPDATE polls SET onchain_status='failed', onchain_tx_hash=NULL, onchain_at=UNIX_TIMESTAMP() WHERE id=?")
                .bind(uid)
                .execute(&st.pool).await?;
            Err(ApiError(axum::http::StatusCode::BAD_GATEWAY, format!("push failed: {e}")))
        }
    }
}
