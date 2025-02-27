use rocket::Responder;
use rocket::serde::Serialize;


/*
Define types of responses the request guard can throw back at us.
Used the following documentation to implement it:
https://medium.com/@jeynesbrook/jwt-authentication-for-api-routes-using-rocket-rs-and-rust-fe7529792a70
*/

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}


#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}