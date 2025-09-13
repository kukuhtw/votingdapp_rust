use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteIdx {
    pub tx_hash: String,
    pub poll_id: String,
    pub voter: String,
    pub option_index: i32,
    pub amount: String,
    pub block_time: i64,
    pub chain_id: String,
}
