use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::Request;
use serde::Serialize;
use std::io::Cursor;

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_in: i64, // Example: expiration time in seconds
}

pub struct AuthResponder {
    pub token: String,
    pub status: Status,
    pub expires_in: i64,
}

impl<'r> Responder<'r, 'static> for AuthResponder {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'static> {
        let body = serde_json::to_string(&AuthResponse {
            token: self.token,
            expires_in: self.expires_in,
        })
        .map_err(|_| rocket::http::Status::InternalServerError)?;

        Response::build()
            .status(self.status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
