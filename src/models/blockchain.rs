use crate::models::block::Block;
use crate::models::transaction::Transaction;

pub struct Blockchain {
    blocks: Vec<Block>,
    difficulty: usize,
}
impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blocks: Vec<Block> = Vec::new();
        let first_block = Block::new("0".to_string(), Vec::new(), difficulty.clone());
        blocks.push(first_block);
        Self {
            blocks,
            difficulty,
        }
    }
}