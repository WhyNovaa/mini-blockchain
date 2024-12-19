use crate::models::block::Block;
use crate::models::blockchain_errors::BlockError;
use crate::models::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
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

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn increase_difficulty(&mut self) {
        self.difficulty += 1;
    }

    //pub fn add_block_(&self) {}
}