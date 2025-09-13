// backend/src/models/poll.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollDto {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub start: Option<i64>,
    pub end: i64,
    pub vote_price: String,
    pub status: String, // "draft" | "published" | "archived"
    // NEW
    pub onchain_status: String,               // "none" | "pending" | "success" | "failed"
    pub onchain_tx_hash: Option<String>,
    pub onchain_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePollReq {
     pub slug: Option<String>,            // ⬅ optional
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub start: Option<i64>,
    pub end: i64,
    pub vote_price: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePollReq {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub options: Option<Vec<String>>,
    pub start: Option<Option<i64>>,
    pub end: Option<i64>,
    pub vote_price: Option<String>,
    pub status: Option<String>,
}
