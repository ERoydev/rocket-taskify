use rocket::{serde::json::Json, State};
use rocket::{delete, get, post};
use sea_orm::*;


use crate::entities::{prelude::*, task};
use crate::entities::task::Entity as TaskEntity;

use crate::interfaces::new_task::NewTask;
use crate::interfaces::task_dto::{TaskDTO, ModelTypes};
use crate::interfaces::task_priority::TaskPriorityLevel;

use crate::ErrorResponder;
use crate::resources::base_sql::get_base_sql;


#[get("/tasks?<sort>")] // Todo retrive by due_date and priority
pub async fn get_tasks(sort: Option<String>, db: &State<DatabaseConnection>) -> Result<Json<Vec<TaskDTO>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let base_sql = get_base_sql();

    let order_clause = match sort {
        Some(s) => s.to_lowercase(),
        None => "%".to_string()
    };

    let sql_query = format!("{} WHERE priority ILIKE '{}' ORDER BY priority_order DESC, due_date ASC;", base_sql, order_clause); // This ensure to add priority=high if sort is provided in the request

    let tasks = TaskEntity::find()
    // This Returns All tasks sorted by priority DESC then by due_date ASC
    .from_raw_sql(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        sql_query,
        [],
    ))
    .all(db)
    .await
    .map_err(Into::<ErrorResponder>::into)?;

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

#[get("/tasks/<id>")]
pub async fn get_task_by_id(id: i32, db: &State<DatabaseConnection>) -> Result<Json<TaskDTO>, ErrorResponder> {
    let db = db  as &DatabaseConnection;

    let task = TaskEntity::find_by_id(id)
        .one(db)
        .await?;

    match task {
        Some(task) => {
            let task_dto: TaskDTO = TaskDTO::initialize(ModelTypes::TaskModel(task), None);

            return Ok(Json(task_dto))
        }
        None => {
            return Err(ErrorResponder::from("There is no task with this id"))
        }
    }
}

#[delete("/tasks/<id>")]
pub async fn delete_task(id: i32, db: &State<DatabaseConnection>) -> Result<String, ErrorResponder> {
    let db = db as &DatabaseConnection;


    let task = TaskEntity::find_by_id(id)
        .one(db)
        .await?;

    match task {
        Some(_task) => {
            TaskEntity::delete_by_id(id)
                .exec(db)
                .await?;

            Ok("Task successfully deleted.".to_string())
        }
        None => {
            return Err(ErrorResponder::from("There is no task with this id"))
        }
    }
    
}