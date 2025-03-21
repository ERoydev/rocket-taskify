use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::{Method, Status};

#[derive(Default)]
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_raw_header("Access-Control-Allow-Origin", "*");
        response.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
        response.set_raw_header("Access-Control-Allow-Headers", "Content-Type");

        // ✅ When i try to DELETE ruquest from frontend it is sended as OPTIONS instead of DELETE
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
    }
}