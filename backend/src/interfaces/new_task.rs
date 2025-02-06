use rocket::serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub due_date: i32, // i use UNIX-Timestamp
    pub is_completed: bool,
    pub is_critical: bool,
}