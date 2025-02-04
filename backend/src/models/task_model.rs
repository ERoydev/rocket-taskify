

#[derive(Debug, Clone, Copy, Default)]
pub enum Priority {
    High,
    Medium,
    #[default]
    Low
}

#[derive(Debug, Clone, Default)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub due_date: i32,
    pub is_completed: bool,
}

impl Task {
    pub fn new(id: i32, title: String, description: String, priority: Priority, due_date: i32, is_completed: bool) -> Task {
        Task {
            id,
            title,
            description,
            priority,
            due_date,
            is_completed
        }
    }
}