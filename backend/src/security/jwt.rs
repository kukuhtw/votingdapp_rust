/*
backend/src/security/jwt.rs
*/


use anyhow::Result;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // admin id (uuid string)
    pub exp: i64,
}

pub fn issue(sub: String, secret: &str, ttl_minutes: i64) -> Result<String> {
    let exp = (OffsetDateTime::now_utc() + Duration::minutes(ttl_minutes)).unix_timestamp();
    let claims = Claims { sub, exp };
    Ok(encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret.as_bytes()))?)
}

pub fn validate(token: &str, secret: &str) -> Result<Claims> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256))?;
    Ok(data.claims)
}
