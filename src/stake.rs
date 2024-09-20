use std::collections::HashMap;


pub struct Stake {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, u64>,
}

impl Stake {
    pub fn new() -> Self {
        Self {
            accounts: vec![],
            balances: HashMap::from([]),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0);
            self.accounts.push(address.to_string());
        }
    }

    pub fn add_stake(&mut self, from: &String, amount: &u64) {
        self.initialize(from);
        *self.balances.get_mut(from).unwrap() += amount;
    }

    pub fn get_max(&mut self, addresses: &Vec<String>) -> String {
        let key = self
            .balances
            .iter()
            .filter(|addr| addresses.contains(&addr.0))
            .collect::<HashMap<_, _>>();
        key.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .to_string()
    }

    pub fn update(&mut self, txn: &Transaction) {
        self.add_stake(&txn.txn_input.from, &(*&txn.txn_output.amount as u64))
    }

    pub fn get_balance(&mut self, address: &String) -> &u64 {
        self.initialize(address);
        self.balances.get(address).unwrap()
    }
}