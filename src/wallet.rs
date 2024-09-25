use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signer}; // helps with keypair generation


pub struct Wallet {
    pub key_pair: String,
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
}