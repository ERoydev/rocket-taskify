// This is my DATA TRANSFER OBJECT for Task Model. Idea is to simplify my data to provide in frontend clean representation

use serde::{Deserialize, Serialize};

use crate::{entities::task, NewTask};
use chrono::DateTime;

use super::task_priority::get_priority_level;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: String, // i use UNIX-Timestamp
    pub is_completed: bool,
    pub is_critical: bool, // If user update
    pub due_date_timestamp: i64,
}

pub enum ModelTypes {
    NewTask(NewTask), // Used to convert NewTask with task_id provided from outside to create TaskDTO
    TaskModel(task::Model), // Used from task::Model to create TaskDTO object
}

impl TaskDTO {
    pub fn initialize(model: ModelTypes, task_id: Option<i32>) -> TaskDTO {
        let task_id = task_id.unwrap_or_default();
        
        match model {
            ModelTypes::TaskModel(model) => {
                // I use this when i Get all Tasks. I convert each Model into TaskDTO model to return
                let converted_due_date = Self::convert_unix_timestamp(Some(model.due_date));

                TaskDTO {
                    id: model.id,
                    title: model.title,
                    description: model.description,
                    priority: model.priority,
                    due_date: converted_due_date,
                    is_completed: model.is_completed,
                    is_critical: model.is_critical,
                    due_date_timestamp: model.due_date.into(),
                }
            }
            ModelTypes::NewTask(model) => {
                // I use this when create a new task. I do priority calculations here
                let converted_due_date: String = Self::convert_unix_timestamp(Some(model.due_date));
                let priority = get_priority_level(model.is_completed, model.is_critical, model.due_date);

                TaskDTO {
                    id: task_id,
                    title: model.title.clone(),
                    description: model.description.clone(),
                    priority: priority,
                    due_date: converted_due_date,
                    is_completed: model.is_completed,
                    is_critical: model.is_critical,
                    due_date_timestamp: model.due_date.into(),
                }
            }
        }
    }

    fn convert_unix_timestamp(timestamp: Option<i64>) -> String {
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
