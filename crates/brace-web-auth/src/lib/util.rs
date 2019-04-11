use std::iter::repeat;
use std::string::FromUtf8Error;

use argon2rs::verifier::DecodeError;
use argon2rs::verifier::Encoded;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn salt() -> String {
    repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .take(32)
        .collect()
}

pub fn hash(password: &str) -> Result<String, FromUtf8Error> {
    String::from_utf8(Encoded::default2i(password.as_bytes(), salt().as_bytes(), &[], &[]).to_u8())
}

pub fn verify(password: &str, hash: &str) -> Result<bool, DecodeError> {
    match Encoded::from_u8(hash.as_bytes()) {
        Ok(enc) => Ok(enc.verify(password.as_bytes())),
        Err(err) => Err(err),
    }
}
