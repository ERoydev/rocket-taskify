use rocket::{serde::json::Json, State};
use sea_orm::*;


use crate::entities::{prelude::*, task};
use crate::entities::task::Entity as TaskEntity;

use crate::interfaces::new_task::NewTask;
use crate::interfaces::task_dto::{TaskDTO, ModelTypes};
use crate::interfaces::task_priority::TaskPriorityLevel;
use crate::ErrorResponder;

use rocket::{get, post};


#[get("/tasks")] // Todo retrive by due_date and priority
pub async fn get_tasks(db: &State<DatabaseConnection>) -> Result<Json<Vec<TaskDTO>>, ErrorResponder> {
    let db = db as &DatabaseConnection;
    
    let tasks = TaskEntity::find()
        .all(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?; // EXPLAIN THIS
    
    // I iterate through Model and convert the it to TaskDTO where i change due_date to string and add new field (Have in mind when you try to deserialize the Model)
    let task_dtos: Vec<TaskDTO> = tasks.into_iter().map(|task| TaskDTO::initialize(ModelTypes::TaskModel(task), None)).collect();
    Ok(Json(task_dtos))
}

#[post("/tasks", format="json", data="<new_task>")]
pub async fn create_task(db: &State<DatabaseConnection>, new_task: Json<NewTask>) -> Result<Json<TaskDTO>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let priority = TaskPriorityLevel::get_priority(&new_task); // Get priority String representation

    let new_task_model = task::ActiveModel {
        title: ActiveValue::Set(new_task.title.clone()),
        description: ActiveValue::Set(new_task.description.clone()),
        priority: ActiveValue::Set(priority), // Todo logic
        due_date: ActiveValue::Set(new_task.due_date),
        is_completed: ActiveValue::Set(new_task.is_completed),
        id: NotSet,
        ..Default::default()
    };

    let insert_result = Task::insert(new_task_model)
        .exec(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;


    let task_dto = TaskDTO::initialize(ModelTypes::NewTask(new_task.into_inner()), Some(insert_result.last_insert_id));

    Ok(Json(task_dto))
}