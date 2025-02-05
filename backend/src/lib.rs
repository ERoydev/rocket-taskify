
pub mod api; // Module for all my API Endpoints
pub mod entities;

pub mod interfaces;
pub use interfaces::new_task::NewTask; // Change if grows
pub use interfaces::task_dto::TaskDTO;

mod error_responder;
pub use error_responder::ErrorResponder;


pub use rocket;
pub mod setup;