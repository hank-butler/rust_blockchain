use ed25519_dalek::{ed25519::Error, PublicKey, Signature, Verifier};
use hex::FromHexError;
use log::{warn, info};
use sha2::{Sha256, Digest};
use uuid::Uuid;

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

    // pub f
}