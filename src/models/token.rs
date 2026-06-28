use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub id: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub token_id:Uuid,
    pub user_pid: Uuid,
    pub expires_in: Option<i64>,
}