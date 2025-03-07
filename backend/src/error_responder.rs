use rocket::{http::Status, response::Responder, Request, Response};
use sea_orm::DbErr;
use std::io::Cursor;

/*
This is my custom Error Responder created from the seaORM Tutorials Documentation

Used to catch and handle errors for example when i create_task i return it with

        Return((), ErrorResponder) 
*/


#[derive(Debug)]
pub struct ErrorResponder {
    message: String,
    http_status: Status, // STORE HTTP STATUS
}


// Example of how i use .new() function bello
// argon2
//     .verify_password(entered_password.as_bytes(), &parsed_hash)
//     .map_err(|_| ErrorResponder::new("Invalid credentials", Status::Unauthorized))

impl ErrorResponder {
    pub fn new(message: &str, status: Status) -> ErrorResponder { // Used to create custom error responses like the example above
        ErrorResponder {
            message: message.to_string(),
            http_status: status,
        }
    }
}

// Implement Responder to return custom status codes
impl<'r> Responder<'r, 'static> for ErrorResponder {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'static> {
        Response::build()
            .status(self.http_status) // Set the custom status
            .header(rocket::http::ContentType::JSON)
            .sized_body(self.message.len(), Cursor::new(self.message))
            .ok()
    }
}

// Implement From traits for easy conversion of errors

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: format!(
                "Database error: Unable to complete the operation. DATABASE ERROR: {}",
                err
            ),
            http_status: Status::InternalServerError, // Default 500
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder {
            message: string,
            http_status: Status::BadRequest, // Default to 400 Bad Request
        }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        ErrorResponder {
            message: str.to_owned(),
            http_status: Status::BadRequest, // Default to 400 Bad Request
        }
    }
}