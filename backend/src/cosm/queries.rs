/*
backend/src/cosm/queries.rs
*/

use anyhow::Result;
use serde_json::{json, Value};
use super::client::smart_query;

pub async fn query_poll(rest: &str, voting: &str, poll_id: &str) -> Result<Value> {
    let msg = json!({ "poll": { "poll_id": poll_id } });
    smart_query(rest, voting, &msg).await
}

pub async fn query_result(rest: &str, voting: &str, poll_id: &str) -> Result<Value> {
    let msg = json!({ "result": { "poll_id": poll_id } });
    smart_query(rest, voting, &msg).await
}
