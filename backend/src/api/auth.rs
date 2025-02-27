
use rocket::post;
use rocket::{serde::json::Json, State};

use crate::ErrorResponder;

#[post("/auth/signup")]
pub async fn signup() -> Result<(), ErrorResponder> {

    Ok(())
}