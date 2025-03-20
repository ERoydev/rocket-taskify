use crate::error_responder::ErrorResponder;

use super::jwt::{decode_jwt, Claims};
use super::utils::is_token_revoked;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use sea_orm::DatabaseConnection;

/*
Module responsible for Request Guards for authentication purposes only.
*/


// Struct to store authenticated user details
#[derive(Debug)]
pub struct AuthenticatedUser { // This is like request Guard for my routes
    pub claims: Claims,
    pub token: String,
}

/*
This is the request Guard when i use it as parameter in API function like
user: AuthenticatedUser 
- It will trigger the middleware that is going to look for Authorization header.
- It will decode the token and check if it's valid or expired.    

----------------------------------
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

    type Error = ErrorResponder;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ErrorResponder> {
        // 1️. Get the Authorization header
        let auth_header = req.headers().get_one("Authorization");

        //  2. Get the database connection from Rocket's state
        let db = match req.rocket().state::<DatabaseConnection>() {
            Some(db) => db,
            None => {
                let response = ErrorResponder::new("Database connection error", Status::InternalServerError);
                return request::Outcome::Error((Status::InternalServerError, response));
            }
        };

        match auth_header {
            Some(token) => {

                let token = token.trim_start_matches("Bearer").trim().to_string(); // Remove `Bearer ` prefix

                // 🔹 3️⃣ Check if the token is revoked
                if is_token_revoked(db, &token).await {
                    let response = ErrorResponder::new("Token has been revoked", Status::Unauthorized);
                    return request::Outcome::Error((Status::Unauthorized, response));
                }

                match decode_jwt(token.to_string()) {
                    Ok(claims) => {
                        request::Outcome::Success( 
                            AuthenticatedUser {
                                claims, 
                                token,
                            })
                    }

                    Err(err) => match &err {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {                            
                            let response = ErrorResponder::new("Error validating JWT token - Expired Token", Status::Unauthorized);

                            request::Outcome::Error((Status::Unauthorized, response))
                        },
                        jsonwebtoken::errors::ErrorKind::InvalidToken => {
                            // let response = Response{body: ResponseBody::Message(format!("Error validating JWT token - Invalid Token"))};

                            let response = ErrorResponder::new("Error validating JWT token - Invalid Token", Status::Unauthorized);

                            request::Outcome::Error((Status::Unauthorized, response))

                        }
                        _ => {
                            let response = ErrorResponder::new(&format!("Error validating JWT token - {:?}", err), Status::Unauthorized);

                            request::Outcome::Error((Status::Unauthorized, response))
                        }
                    }
                }
            }
            None => {
                let response = ErrorResponder::new("Error validating JWT token - Expired Token", Status::Unauthorized);

                request::Outcome::Error((Status::Unauthorized, response))
            }
        }
    }
}