// need to bring in transaction here
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transaction: Vec<Transaction>,
    pub signature: String,
    pub difficulty: usize, 
}

impl Block {
    pub fn new (
        id: usize,
        previous_hash: String,
        timestamp: i64,
        transaction: Vec<Transaction>,
        difficulty: usize,
        mut validator_wallet: Wallet,
    ) -> Self {
        info!("New block created at {}", timestamp);

        let hash = Block::calculate_hash(&id, &timestamp, &previous_hash, &transaction); // need to work on this.

        let signature = validator_wallet.sign(&hash); // need sign method for validator_wallet

        Self {
            id,
            hash, 
            previous_hash,
            timestamp,
            transaction,
            // validator, need to work on this part
            signature,
            difficulty,
        }
    }
}