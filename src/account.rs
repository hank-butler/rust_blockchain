use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, f64>,
}

impl Account {
    pub fn new() -> Self {
        Self {
            accounts: vec![], // do I need to simulate with new accounts?
            balances: HashMap::from([]) // add balances to them at execution?
        }
    }
    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0.00);
            self.accounts.push(address.to_string());
        }
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: &f64) {
        self.initialize(from);
        self.initialize(to);
        self.increment(from, to, amount);
        self.decrement(from, to, amount);
    }

    pub fn increment(&mut self, from: &String, to:&String, amount: &fg4) {
        (*self.balances.get_mut(to).unwrap()) += amount;
    }

    pub fn decrement(&mut self, from: &String, to: &String, amount: &f64) {
        (*self.balances.get_mut(from).unwrap()) -= amount;
    }

    pub fn get_balance(&mut self, address: &String) -> f64 {
        self.initialize(address);
        self.balances.get(address).unwrap();
    }

}