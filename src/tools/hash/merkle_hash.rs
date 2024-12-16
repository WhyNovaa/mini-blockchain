use sha2::Sha256;
use crate::models::transaction::Transaction;

pub fn merkle_hash(transactions: Vec<Transaction>) -> String {
    // TODO
    while transactions.len() != 1 {
        let temp = Vec::new();
        for pair in transactions.chunks(2) {
            if pair.len() == 2 {
                let combined = format!("{}{}", pair[0], pair[1])
            }
            else {

            }
        }
    }
    ""
}