use serde::Serialize;
use sha2::Sha256;
use crate::tools::hash::hash::hash;

#[derive(Serialize)]
pub struct Transaction {
    pub amount: u128,
    pub from: String,
    pub to: String,
}

impl Transaction {
    pub fn new(amount: u128, from: String, to: String) -> Self {
        Self {
            amount,
            from,
            to,
        }
    }

    pub fn hash(&self) -> Result<String, serde_json::error::Error> {
        let serialized = serde_json::to_string(self)?;
        Ok(hash(serialized.as_bytes()))
    }
}