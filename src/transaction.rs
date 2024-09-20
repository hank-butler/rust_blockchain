


pub const TRANSACTION_FEE: f64 = 1.0;

pub enum TransactionType {
    TRANSACTION,
    STAKE,
    VALIDATOR
}

pub struct TransactionInput {
    pub timestamp: i64,
    pub from: String,
    pub signature: String,
}

impl TransactionInput {
    pub fn new(
        sender_waller: &mut Wallet,
        transaction_output: &String
    ) -> Self {
    Self {timestamp: Utc::now().timestamp(),
        from: sender_waller.get_public_key(),
        signature: sender_wallet.sign(transaction_output),
        }
    }
}

pub struct Transaction {
    // need to build out this struct
    pub id: usize,
    pub transaction_type: TransactionType, 
    pub transaction_input: TransactionInput,
    pub transaction_output: TransactionOutput,
    // sender, receiver, timestamp, amount
}

pub struct TransactionOutput {
    pub to: String,
    pub amount: f64,
    pub fee: f64,
}

impl TransactionOutput {
    pub fn new(
        to: String, 
        amount: f64,
        fee: f64
    ) -> Self {
        Self {
            to: to,
            amount: amount,
            fee: fee
        }
    }
}