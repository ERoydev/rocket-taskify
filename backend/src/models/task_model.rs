

pub enum Priority {
    High,
    Medium,
    Low
}

pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub due_date: i32,
    pub is_completed: bool,
}

