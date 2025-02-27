use rocket::{post, get};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use jsonwebtoken::{Algorithm, Header, encode, EncodingKey};
use jsonwebtoken::errors::Error;
use dotenv::dotenv;

use crate::ErrorResponder;
use std::env;

/*
Used the following documentation as a guidance for creating the JWT authentication logic
    - url: https://medium.com/@jeynesbrook/jwt-authentication-for-api-routes-using-rocket-rs-and-rust-fe7529792a70
*/

const TOKEN_EXPIRE_TIME_IN_SECONDS: i64 = 60; // i64 expected from chrono::Duration::seconds
const USED_HASH_ALGORITHM: Algorithm = Algorithm::HS512;
/*
HMAC(hash message authentication code) using SHA-512 (512bits)
It's a symmetric encryption algorithm:
Risks:
    - Both the server and clients need to use the same key for (encryption and decription)
Solution:
    - I will use ECDH Public key which clients sends with his request to register
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    // Struct that JWT will use to encode
    pub subject_id: i32, // Corresponds to id of the user who created the token
    exp: usize // Represents how long the token has to live
}

#[derive(Debug)]
pub struct JWT {
    // Struct for the token itself
    pub claims: Claims
}

pub fn create_jwt(id: i32) -> Result<String, Error> {
    // Takes the user id and returns JSON string containing the JWT or jsonwebtoken Error.
    dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET MUST BE SET");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(TOKEN_EXPIRE_TIME_IN_SECONDS))
        .expect("Invalid timestamp").timestamp(); // Variable holding current time and then add additional EXPIRE_TIME seconds using chrono

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize
    };

    let header = Header::new(USED_HASH_ALGORITHM);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, Error> {
    let token = token.trim_start_matches("Bearer").trim();

    
    todo!()
}


#[post("/auth/signup/<id>")]
pub async fn signup(id: i32) -> Result<(), ErrorResponder> {
    let jwt = create_jwt(id).unwrap();

    println!("JWT RESULT: {}", jwt);

    Ok(())
}