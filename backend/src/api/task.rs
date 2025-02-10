
use rocket::{serde::json::Json, State};
use rocket::{delete, get, post, put};

use sea_orm::*;


use crate::entities::{prelude::*, task};
use crate::entities::task::Entity as TaskEntity;

use crate::interfaces::new_task::NewTask;
use crate::interfaces::task_dto::{TaskDTO, ModelTypes};
use crate::interfaces::task_priority::get_priority_level;

use crate::ErrorResponder;
use crate::resources::base_sql::get_base_sql;


// GET ALL TASKS SORTED BY (PRIORITY, DUE_DATE) OR GET TASK BY PRIORITY LEVEL
// GET /tasks?sort=high or GET /tasks
#[get("/tasks?<sort>", rank=2)] // Todo retrive by due_date and priority
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


    /* Note: This is inefficient because i iterate through all tasks to initialize them in Task Struct and could be very slow if records where 1 million for example. 
        + But since its Task application this isn't big problem, because every user will request his own tasks which is aren't going to be even above 100.
        + Otherwise i could fix my problem using DB Stored Procedure to do all priority calculations and unix-timestamp conversion.
    */ 
    // I iterate through Model and convert the it to TaskDTO where i change due_date to string and add new field (Have in mind when you try to deserialize the Model)
    let task_dto_vector: Vec<TaskDTO> = tasks.into_iter().map(|task| TaskDTO::initialize(ModelTypes::TaskModel(task), None)).collect();

    Ok(Json(task_dto_vector))
}


// GET /tasks?filter=isCompleted&value=true Get task by completion property
#[get("/tasks?<filter>&<value>")]
pub async fn get_tasks_by_completion_status(filter: String, value: String, db: &State<DatabaseConnection>) -> Result<Json<Vec<TaskDTO>>, ErrorResponder> {
    let _ = filter;
    
    let db = db as &DatabaseConnection;
    
    let value_bool = match value.to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        "" => return Err(ErrorResponder::from("Value parameter cannot be empty")),      
        _ => return Err(ErrorResponder::from("Value must be true or false")),
    };

    let tasks = TaskEntity::find()
        .filter(task::Column::IsCompleted.eq(value_bool))
        .all(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;

    let task_dto: Vec<TaskDTO> = tasks.into_iter().map(|task| TaskDTO::initialize(ModelTypes::TaskModel(task), None)).collect();
    Ok(Json(task_dto))
}


// GET TASK BY ID
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


#[post("/tasks", format="json", data="<new_task>")]
pub async fn create_task(db: &State<DatabaseConnection>, new_task: Json<NewTask>) -> Result<Json<TaskDTO>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let priority = get_priority_level(new_task.is_completed, new_task.is_critical, new_task.due_date);

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


#[put("/tasks", format="json", data="<updated_task>")]
pub async fn update_task(db: &State<DatabaseConnection>, updated_task: Json<TaskDTO>) -> Result<Json<TaskDTO>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let priority = get_priority_level(updated_task.is_completed, updated_task.is_critical, updated_task.due_date_timestamp);

    let updated_task_model = task::ActiveModel {
        title: ActiveValue::Set(updated_task.title.clone()),
        description: ActiveValue::Set(updated_task.description.clone()),
        priority: ActiveValue::Set(priority), // Todo logic
        due_date: ActiveValue::Set(updated_task.due_date_timestamp ),
        is_completed: ActiveValue::Set(updated_task.is_completed),
        id: ActiveValue::Set(updated_task.id),
        ..Default::default()
    };

    let task_model = updated_task_model
        .update(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;

    let task_dto: TaskDTO = TaskDTO::initialize(ModelTypes::TaskModel(task_model), None);

    Ok(Json(task_dto))
}


#[delete("/tasks/<id>")]
pub async fn delete_task(id: i32, db: &State<DatabaseConnection>) -> Result<Json<String>, ErrorResponder> {

    let db = db as &DatabaseConnection;

    let task = TaskEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?;

    match task {
        Some(_task) => {
            TaskEntity::delete_by_id(id)
                .exec(db)
                .await?;

            Ok(Json("Task successfully deleted.".to_string()))
        }
        None => {
            return Err(ErrorResponder::from("There is no task with this id"))
        }
    }
    
}


#[post("/tasks/complete/<id>")]
pub async fn complete_task(id: i32, db: &State<DatabaseConnection>) -> Result<Json<TaskDTO>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let task = TaskEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?
        .ok_or_else(|| ErrorResponder::from("You cannot complete a task which doesn't exists, try using another task id!"))?;


    let mut task_active_model: task::ActiveModel = task.into();
    task_active_model.is_completed = Set(true);
    task_active_model.is_critical = Set(false);

    let priority = get_priority_level(task_active_model.is_completed.clone().unwrap(), false, task_active_model.due_date.clone().unwrap()); // Because when it is completed i cannot be critical anymore

    task_active_model.priority = Set(priority);

    let task: task::Model = task_active_model.update(db).await?;

    let task_dto: TaskDTO = TaskDTO::initialize(ModelTypes::TaskModel(task), None);

    Ok(Json(task_dto))
}

#[post("/tasks/critical/<id>")]
pub async fn critical_task(id: i32, db: &State<DatabaseConnection>) -> Result<Json<TaskDTO>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let task = TaskEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?
        .ok_or_else(|| ErrorResponder::from("You cannot set critical to task that doesn't exists, try using another task id!"))?;

    let mut task_active_model: task::ActiveModel = task.into();
    task_active_model.is_critical = Set(true);

    let priority = get_priority_level(task_active_model.is_completed.clone().unwrap(), task_active_model.is_critical.clone().unwrap(), task_active_model.due_date.clone().unwrap());

    task_active_model.priority = Set(priority);

    let task: task::Model = task_active_model.update(db).await?;

    let task_dto: TaskDTO = TaskDTO::initialize(ModelTypes::TaskModel(task), None);

    Ok(Json(task_dto))
}

#[post("/tasks/update_priority")]
pub async fn update_tasks_priority(db: &State<DatabaseConnection>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    // I fetch tasks that need a priority update
    let mut tasks: Vec<task::Model> = TaskEntity::find()
        .filter(
            // I dont want to update priority on tasks which have already defined static priority based on these two boolean scenarios or no need to calculate expired tasks
            Condition::all()
                .add(task::Column::IsCompleted.eq(false)) 
                .add(task::Column::IsCritical.eq(false))
                .add(task::Column::Priority.not_like("%expired%"))
        )
        .into_model()
        .all(db)
        .await?;

      // Prepare a vector to store the updated tasks for the response

    tasks.iter_mut().for_each(|task| {
        task.priority = get_priority_level(false, false, task.due_date);
    });

    for task in tasks {
        let active_model: task::ActiveModel = task.into();

        active_model.update(db).await?;
    }
    
    Ok(())
}