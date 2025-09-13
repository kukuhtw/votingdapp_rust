/*
backend/src/db/redis.rs
*/


use anyhow::Result;
use redis::Client;

pub fn create_client(url: &str) -> Result<Client> {
    let client = redis::Client::open(url)?;
    Ok(client)
}
