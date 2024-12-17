use std::sync::atomic::{AtomicU64, Ordering};
use crate::models::header::Header;
use crate::models::transaction::Transaction;
use crate::tools::hash::hash::hash;
use crate::tools::hash::merkle_hash::merkle_hash;

static NEXT_IND: AtomicU64 = AtomicU64::new(0);
pub struct Block {
    index: u64,
    header: Header,
    hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(parent_hash: String, transactions: Vec<Transaction>, difficulty: usize) -> Self {
        let index = NEXT_IND.fetch_add(1, Ordering::SeqCst);
        let merkle_hash = merkle_hash(transactions.clone());
        let timestamp = chrono::offset::Local::now().timestamp();
        println!("{index}, {merkle_hash}, {timestamp}");
        let (hash, nonce) = Self::mine_block(parent_hash.clone(), merkle_hash.clone(), timestamp, difficulty);

        let header = Header::new(parent_hash, merkle_hash, timestamp, nonce, difficulty);

        println!(
            "Created:\n{index}\n{header}\n{hash}\n{:?}", transactions
        );

        Self {
            index,
            header,
            hash,
            transactions,
        }
    }

    fn mine_block(parent_hash: String, merkle_hash: String, timestamp: i64, difficulty: usize) -> (String, u128){
        let mut nonce: u128 = 0;

        loop {
            println!("{nonce}: Mining...");
            let hash = Self::calculate_hash(parent_hash.clone(), merkle_hash.clone(), timestamp, nonce);

            if Self::is_valid(hash.clone(), difficulty) {
                println!("Found hash: {}, nonce: {}", hash, nonce);
                return (hash, nonce)
            }

            nonce += 1;
        }
    }

    fn calculate_hash(parent_hash: String, merkle_hash: String, timestamp: i64, nonce: u128) -> String {
        let data = format!("{}{}{}{}", parent_hash, merkle_hash, timestamp, nonce);

        hash(data.as_bytes())
    }

    fn is_valid(hash: String, difficulty: usize) -> bool {
        if hash.starts_with(&"0".repeat(difficulty)) {
            return true;
        }
        false
    }
}