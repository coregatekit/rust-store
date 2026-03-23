use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor {
    pub id: i32,
    pub created_at: NaiveDateTime,
}

impl Cursor {
    pub fn encode(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(URL_SAFE_NO_PAD.encode(json))
    }

    pub fn decode(s: &str) -> Result<Self> {
        let bytes = URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|e| anyhow!("invalid cursor encoding: {e}"))?;
        serde_json::from_slice(&bytes).map_err(|e| anyhow!("invalid cursor payload: {e}"))
    }
}
