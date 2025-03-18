use rocket::{post, get};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, decode};
use jsonwebtoken::errors::{Error, ErrorKind};
use dotenv::dotenv;
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

pub struct JwtResponse {
    pub token: String,
    pub exp: i64, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    // Struct that JWT will use to encode PAYLOAD
    pub subject_id: i32, // Corresponds to id of the user who created the token
    exp: u64, // Represents how long the token has to live
}

pub fn create_jwt(id: i32) -> Result<JwtResponse, Error> {
    // Takes the user id and returns JSON string containing the JWT or jsonwebtoken Error.
    dotenv().ok();

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET MUST BE SET");

    let expiration: u64 = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(TOKEN_EXPIRE_TIME_IN_SECONDS))
        .expect("Invalid timestamp").timestamp().try_into().unwrap(); // Variable holding current time and then add additional EXPIRE_TIME seconds using chrono

    // Payload => user related data like user_id, expiration ...
    let claims = Claims {
        subject_id: id,
        exp: expiration,
    };

    // Header containing metadata like the algorithm used
    let header = Header::new(USED_HASH_ALGORITHM);

    // Signature
    let token = encode(
        &header, 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes()))?; // Returns token

    Ok(JwtResponse{
        token, 
        exp: TOKEN_EXPIRE_TIME_IN_SECONDS
    })
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    // Used to decode and authenticate a token when provided

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET MUST BE SET");

    let token = token.trim_start_matches("Bearer").trim(); // I need to extract the token itself

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(USED_HASH_ALGORITHM),
    ) {
        Ok(token) => {
            Ok(token.claims)
        },
        Err(err) => {
            Err(err.kind().to_owned())
        }
    }
}

