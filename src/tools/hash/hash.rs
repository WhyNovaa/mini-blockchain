use std::hash::Hash;
use sha2::{Digest, Sha256};

pub fn hash(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let res_bytes = hasher.finalize();
    let hex: String = res_bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
    hex
}