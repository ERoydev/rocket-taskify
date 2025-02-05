use rocket::{serde::json::Json, State};
use sea_orm::*;


use crate::entities::{prelude::*, task};

use crate::interfaces::new_task::NewTask;
use crate::ErrorResponder;

use rocket::{get, post};


#[get("/tasks")]
pub async fn get_tasks(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;
    
    let tasks = task::Entity::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.title)
        .collect::<Vec<String>>();

    Json(tasks)
}

#[post("/tasks", format="json", data="<new_task>")]
pub async fn create_task(db: &State<DatabaseConnection>, new_task: Json<NewTask>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_task_model = task::ActiveModel {
        title: ActiveValue::Set(new_task.title.clone()),
        description: ActiveValue::Set(new_task.description.clone()),
        priority: ActiveValue::Set(new_task.priority.clone()),
        due_date: ActiveValue::Set(new_task.due_date),
        is_completed: ActiveValue::Set(new_task.is_completed),
        id: NotSet,
        ..Default::default()
    };

    Task::insert(new_task_model)
        .exec(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;

    Ok(())
}