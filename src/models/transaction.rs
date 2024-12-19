use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use crate::tools::hash::hash::hash;

#[derive(Serialize, Clone, Debug)]
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

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Transaction: amount:{}, from:{}, to:{}", self.amount, self.from, self.to)
    }
}