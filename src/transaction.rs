use crate::wallet::Wallet; // why is wallet not being recognized? Update, probably need to actually build out impl
use chrono::prelude::*; // what is prelude?
use serde::{Deserialize, Serialize};
// use serde_json;
// use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;
use crate::util::{Util, VerifySigErr};


pub const TRANSACTION_FEE: f64 = 1.0; // hardcoded for when simulating network

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    TRANSACTION,
    STAKE,
    VALIDATOR
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionInput {
    pub timestamp: i64,
    pub from: String,
    pub signature: String,
}

impl TransactionInput {
    pub fn new(
        sender_wallet: &mut Wallet,
        transaction_output: &String
    ) -> Self {
    Self {timestamp: Utc::now().timestamp(),
        from: sender_wallet.get_public_key(),
        signature: sender_wallet.sign(transaction_output),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    // need to build out this struct
    pub id: usize,
    pub transaction_type: TransactionType, 
    pub transaction_input: TransactionInput,
    pub transaction_output: TransactionOutput,
    // sender, receiver, timestamp, amount
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub enum VerifyTransactionError {
    DecodeJsonErr(serde_json::Error),
    VerifySigErr()
}

impl Transaction {
    pub fn new(
        sender_wallet: &mut Wallet,
        to: String,
        amount: f64,
        transaction_type: TransactionType
    ) -> Result<Self, serde_json::Error> {
        let transaction_output = TransactionOutput::new(to, amount, TRANSACTION_FEE);
        
        let serialized = match serde_json::to_string(&transaction_output) {
            Ok(serialized) => serialized,
            Err(e) => return Err(e),
        };

        let transaction_input = TransactionInput::new(sender_wallet, &serialized);

        // can remove : up to , don't need variable type if same name as parameter
        Ok(Self {
            id,
            transaction_type: transaction_type,
            transaction_output: transaction_output,
            transaction_input: transaction_input,
        })
    }

    pub fn verify_transaction(transaction: &Transaction) -> Result<bool, VerifyTransactionError> {
        let transaction_message = match serde_json::to_string(&transaction.transaction_output) {
            Ok(transaction_message) => transaction_message,
            Err(e) => return Err(VerifyTransactionError::DecodeJsonErr(e)),
        };

        let result = match Util::verify_signature(
            &transaction.transaction_input.from,
            &transaction_message,
            &transaction.transaction_input.signature,
        ) {
            Ok(result) => result,
            Err(e) => match e {
                VerifySigErr::DecodeStrError(_) => false,
                VerifySigErr::DecodeHexError(_) => false,
            };
            };
        }

        Ok(result)
}

    
