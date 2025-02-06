use rocket::serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: i32, // i use UNIX-Timestamp
    pub is_completed: bool,
}