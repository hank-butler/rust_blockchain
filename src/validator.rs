use crate::transaction::Transaction;

pub struct Validator {
    pub accounts: Vec<String>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            accounts: vec![],
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
