use std::path::PathBuf;

use jsonwebtoken::{DecodingKey,EncodingKey};
use serde::Deserialize;

use crate::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct RsaJwtConfig {
    private_key: PathBuf,
    public_key: PathBuf,
    exp: i64,
}

impl RsaJwtConfig {

   pub fn encoding_key(&self) -> Result<EncodingKey> {
    println!("Opening private key: {:?}", self.private_key);

    let contents = std::fs::read_to_string(&self.private_key)?;

    println!("Private key loaded");

    EncodingKey::from_rsa_pem(contents.as_bytes()).map_err(Into::into)
    }

    pub fn decoding_key(&self) -> Result<DecodingKey> {
    println!("Opening public key: {:?}", self.public_key);

    let contents = std::fs::read_to_string(&self.public_key)?;

    println!("Public key loaded");

    DecodingKey::from_rsa_pem(contents.as_bytes()).map_err(Into::into)
    }

    pub fn exp(&self) -> i64 {
        self.exp
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    access: RsaJwtConfig,
    refresh: RsaJwtConfig,
}

impl AuthConfig {
    pub fn access(&self) -> &RsaJwtConfig {
        &self.access
    }

    pub fn refresh(&self) -> &RsaJwtConfig {
        &self.refresh
    }
}