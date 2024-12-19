use std::fmt;
use std::fmt::Formatter;

pub struct Header {
    pub parent_hash: String,
    pub merkle_hash: String,
    pub timestamp: i64,
    pub nonce: u128,
    pub difficulty: usize,
}

impl Header {
    pub fn new(parent_hash: String, merkle_hash: String, timestamp: i64, nonce: u128, difficulty: usize) -> Self {
        Self {
            parent_hash,
            merkle_hash,
            timestamp,
            nonce,
            difficulty,
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\t\tparent_hash: {},\n \t\tmerkle_hash: {},\n \t\ttimestamp: {},\n \t\tnonce: {},\n \t\tdifficulty: {},\n",
               self.parent_hash, self.merkle_hash, self.timestamp, self.nonce, self.difficulty
        )
    }
}