use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultCache {
    pub poll_id: String,
    pub tallies: Vec<String>,
    pub total: String,
    pub updated_at: i64,
}
