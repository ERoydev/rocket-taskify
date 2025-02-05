
pub mod api; // Module for all my API Endpoints
mod entities;

mod interfaces;
pub use interfaces::new_task::NewTask; // Change if grows

mod error_responder;
pub use error_responder::ErrorResponder;

pub use rocket;
pub mod setup;