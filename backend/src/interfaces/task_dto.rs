// This is my DATA TRANSFER OBJECT for Task Model

use serde::{Deserialize, Serialize};

use crate::entities::task;
use chrono::DateTime;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: String, // i use UNIX-Timestamp
    pub is_completed: bool,
    pub due_date_timestamp: i64,
}


impl TaskDTO {
    pub fn initialize(model: task::Model) -> TaskDTO {

        let converted_due_date = Self::convert_unix_timestamp(Some(model.due_date));

        TaskDTO {
            id: model.id,
            title: model.title,
            description: model.description,
            priority: model.priority,
            due_date: converted_due_date,
            is_completed: model.is_completed,
            due_date_timestamp: model.due_date.into(),
        }
    }

    fn convert_unix_timestamp(timestamp: Option<i32>) -> String {
        match timestamp {
            Some(ts) => {
                let datetime = DateTime::from_timestamp(ts as i64, 0)
                    .expect("Invalid timestamp");
                datetime.format("%d-%m-%y").to_string()
            }
            None => String::from("N/A"),  // Handle the None case if necessary
        }
    }
}
