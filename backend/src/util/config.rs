/*
backend/src/util/config.rs
*/


use anyhow::{anyhow, Result};

pub struct Config {
    pub port: u16,
    pub mysql_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let port = std::env::var("PORT").unwrap_or_else(|_| "8081".into()).parse().unwrap_or(8081);
        let mysql_url = std::env::var("MYSQL_URL").map_err(|_| anyhow!("MYSQL_URL missing"))?;
        let redis_url = std::env::var("REDIS_URL").map_err(|_| anyhow!("REDIS_URL missing"))?;
        let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| anyhow!("JWT_SECRET missing"))?;
        Ok(Self { port, mysql_url, redis_url, jwt_secret })
    }
}
