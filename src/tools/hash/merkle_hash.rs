use crate::models::transaction::Transaction;
use crate::tools::hash::hash::hash;

pub fn merkle_hash(transactions: Vec<Transaction>) -> String {
    if transactions.is_empty() {
        return "0".to_string();
    }

    let mut curr: Vec<String> = transactions
        .iter()
        .map(|tr| hash(
            tr.to_string().as_bytes()
        ))
        .collect();

    while curr.len() != 1 {
        let mut next = Vec::new();
        for pair in curr.chunks(2) {
            if pair.len() == 2 {
                let combined = format!("{}{}", pair[0], pair[1]);
                let bytes = combined.as_bytes();
                next.push(
                    hash(bytes)
                    );
            }
            else {
                let combined = format!("{}{}", pair[0], pair[0]);
                let bytes = combined.as_bytes();
                next.push(
                  hash(bytes)
                )
            }
        }
        curr = next.clone();
    }
    curr[0].clone()
}