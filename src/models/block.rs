use crate::models::header::Header;
use crate::models::transaction::Transaction;

static NEXT_IND: u128 = 0;
pub struct Block {
    index: u128,
    header: Header,
    transactions: Vec<Transaction>,
}

/*impl Block {
    pub fn new(parent_hash: String, transactions: Vec<Transaction>, ) -> Self {
        // TODO
        let header = Header::new(parent_hash, );
        Self {
            NEXT_IND,

        }
    }
}*/