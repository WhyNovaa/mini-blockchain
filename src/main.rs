extern crate core;

use crate::models::blockchain::Blockchain;

mod models;
mod tools;

fn main() {
    let blockchain = Blockchain::new(1);
    let blocks = blockchain.blocks;
    println!("{}", blocks[0]);

}
