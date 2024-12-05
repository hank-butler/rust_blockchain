use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signer}; // helps with keypair generation
use crate::blockchain::Blockchain;

pub struct Wallet {
    pub keypair: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let mut csprng = OsRng {}; // need to generate psuedo-random numbers for keypairs
        let keypair = Keypair::generate(&mut csprng);

        let public_key = hex::encode(keypair.public.to_bytes());

        println!("Public Key: {}", public_key);

        let keypair = hex::encode(keypair.to_bytes());

        println!("Keypair: {}", keypair);

        Self { keypair }
    }

    pub fn generate_wallet() {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let public_key = hex::encode(keypair.public.to_bytes());

        println!("Remember to keep your keys safe!");
        println!("Public Key: {}", public_key);
        println!("Key pair: {}", keypair);

    }

    pub fn get_keypair(keypair_string: &String) -> Keypair {
        Keypair::from_bytes(&hex::decode(keypair_string).expect("Expected hex to bytes conversion"))
            .expect("Byte to Keypair Conversion")
    }

    pub fn get_wallet(keypair: String) -> Wallet {
        Self {keypair}
    }

    pub fn sign(&mut self, data_hash: &String) -> String {
        hex::encode(Wallet::get_keypair(&self.keypair).sign(data_hash.as_bytes()))
    }

    pub fn get_public_key(&mut self) -> String {
        hex::encode(Wallet::get_keypair(&self.keypair).public.as_bytes())
    }

    pub fn get_balanca<'a>(&mut self, blockchain: &'a mut Blockchain) -> &'a f64 {
        blockchain.get_balance(&self.get_public_key())
    }
}