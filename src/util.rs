use ed25519_dalek::{ed25519::Error, PublicKey, Signature, Verifier};
use hex::FromHexError;
use log::{warn, info};
use sha2::{Sha256, Digest};
use uuid::Uuid;

pub mod util;

pub struct Util;

pub enum VerifySigErr {
    DecodeStrError(FromHexError),
    DecodeHexError(ed25519_dalek::ed25519::Error)
}

impl From<FromHexError> for VerifySigErr{
    fn from (err: FromHexError) -> Self {
        VerifySigErr::DecodeStrError(err)
    }
}

impl Util {
    pub fn id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn verify_signature(
        from_public_key: &String,
        message: &String,
        from_signature: &String,
    ) -> Result<bool, VerifySigErr> {
        let public_key = hex::decode(from_public_key);
        let dalek_public_key = PublicKey::from_bytes(&public_key)?;

        let signature = hex::decode(from_signature)?;
        let dalek_sig = &Signature::from_bytes(&signature)?;

        Ok(dalek_public_key
            .verify(message.as_bytes(), dalek_sig)
            .is_ok())
    }

    pub fn hash(data: &String) -> String {
        
        let mut hasher = Sha256::default();

        hasher.input(data);

        let output = hasher.result().to_string();

        output
    }
}