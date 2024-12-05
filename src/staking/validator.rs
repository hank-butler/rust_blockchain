use crate::transaction::Transaction;

pub struct Validator {
    pub accounts: Vec<String>, // may need to be a wallet?
}

impl Validator {
    pub fn new() -> Self {
        Self {
            accounts: vec![], // example of PoS I was using hardcoded accounts
        }
    }

    pub fn update(
        &mut self,
        transaction: &Transaction,
        amount: usize) -> bool {
            if transaction.transaction_output.amount >= amount 
            && transaction.transacount_output.to == "0".to_string() {
                self.accounts.push(transaction.transaction_input.from.to_strong());
                true
            }
            false
        }
}
