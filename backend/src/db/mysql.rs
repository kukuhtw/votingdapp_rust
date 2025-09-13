/*
backend/src/db/mysql.rs
*/

use anyhow::Result;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn create_pool(url: &str) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await?;
    Ok(pool)
}
