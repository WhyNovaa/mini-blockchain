pub struct Header {
    pub parent_hash: String,
    pub merkle_hash: String,
    pub timestamp: i64,
    pub nonce: i64,
    pub difficulty: u8,
}

impl Header {
    pub fn new(parent_hash: String, merkle_hash: String, nonce: i64, difficulty: u8) -> Self {
        Self {
            parent_hash,
            merkle_hash,
            timestamp: chrono::offset::Local::now().timestamp(),
            nonce,
            difficulty,
        }
    }
}