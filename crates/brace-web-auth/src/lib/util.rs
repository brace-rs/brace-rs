use std::iter::repeat;
use std::string::FromUtf8Error;

use argon2rs::verifier::{DecodeError, Encoded};
use failure::Error;
use jsonwebtoken::{decode, encode, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::model::{Claims, SlimUser};

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

pub fn create_token(data: SlimUser) -> Result<String, Error> {
    let claims = Claims::with_email(data.email.as_str());
    let res = encode(&Header::default(), &claims, get_secret().as_ref())?;

    Ok(res)
}

pub fn decode_token(token: &str) -> Result<SlimUser, Error> {
    let res = decode::<Claims>(token, get_secret().as_ref(), &Validation::default())?;

    Ok(res.claims.into())
}

fn get_secret() -> String {
    "secret".to_string()
}
