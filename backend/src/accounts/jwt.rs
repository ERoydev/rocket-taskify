use rocket::http::Status;
use rocket::{post, get};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use rocket::request::{self, FromRequest, Request};
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

#[derive(Debug)]
pub struct JWT {
    // Struct for the token itself
    pub claims: Claims
}

// Struct to store authenticated user details
#[derive(Debug)]
pub struct AuthenticatedUser { // This is like request Guard for my routes
    pub user_id: i32,
}

/*
Example of usage:

#[get("/protected")]
fn protected_route(user: AuthenticatedUser) -> String {
    format!("Welcome, User ID: {}", user.user_id)
}

    - Now, only requests with valid JWTs can access /protected.
*/

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    /*
        This plays the role of a middleware(JWT Authentication Guard)
        When a user makes a request, the middleware (JWT Auth Guard) should:

            1.Check if the request has an Authorization: Bearer <token> header.
            2.Verify the JWT token (decode & validate it).
            3.Extract the user’s ID and role from the token.
            4.Allow or deny access based on the token’s validity.
     */

    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // 1️⃣ Get the Authorization header
        let auth_header = req.headers().get_one("Authorization");

        match auth_header {
            Some(token) => {
                // TODO: This logic is not working yet somewhere in decoding error happens
                match decode_jwt(token.to_string()) {
                    Ok(token_data) => request::Outcome::Success(AuthenticatedUser{
                        user_id: token_data.subject_id
                    }),
                    Err(_) => request::Outcome::Error((Status::Unauthorized, ()))
                }
            }
            None => {
                eprintln!("❌ Missing Authorization Header");
                request::Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
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
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))?; // Returns token

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
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}

