// src/main.rs

use rocket::Responder;
use sea_orm::DbErr;


/*
This is my custom Error Responder created from the seaORM Tutorials Documentation

Used to catch and handle errors for example when i create_task i return it with

        Return((), ErrorResponder) 
*/


#[derive(Responder,)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String
}

// The following impl's are for easy conversion of error types.

impl From<DbErr> for ErrorResponder {
    fn from(_err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: "Database error: Unable to complete the operation. Please try again later".to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        ErrorResponder { message: str.to_owned().into() }
    }
}

