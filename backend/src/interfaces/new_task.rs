use rocket::serde::Deserialize;


#[derive(Deserialize)]
#[derive(Debug, Clone)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: i32,
    pub is_completed: bool,
}