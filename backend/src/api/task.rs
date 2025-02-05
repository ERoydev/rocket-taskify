use rocket::{serde::json::Json, State};
use sea_orm::*;


use crate::entities::{prelude::*, task};
use crate::entities::task::Entity as TaskEntity;

use crate::interfaces::new_task::NewTask;
use crate::interfaces::task_dto::TaskDTO;
use crate::ErrorResponder;

use rocket::{get, post};


#[get("/tasks")]
pub async fn get_tasks(db: &State<DatabaseConnection>) -> Result<Json<Vec<TaskDTO>>, ErrorResponder> {
    let db = db as &DatabaseConnection;
    let tasks = TaskEntity::find()
        .all(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;
    
    // I iterate through Model and convert the it to TaskDTO where i change due_date to string and add new field (Have in mind when you try to deserialize the Model)
    let task_dtos: Vec<TaskDTO> = tasks.into_iter().map(|task| TaskDTO::initialize(task)).collect();
    Ok(Json(task_dtos))
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