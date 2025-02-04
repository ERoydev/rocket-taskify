use sea_orm::*;
use rocket::{serde::json::Json, State};

mod setup;
use setup::set_up_db;

mod api;
use api::new_task::NewTask;

mod error_responder;
use error_responder::ErrorResponder;

mod entities;
use entities::{prelude::*, task};


// sea-orm-cli migrate generate create_tasks_table i have created my migration file
// sea-orm-cli migrate up -u postgres://postgres:test123@localhost:5432/rocket_taskify Then i migrate all unapplyed migrations
// sea-orm-cli generate entity -u postgres://postgres:test123@localhost:5432/rocket_taskify -o src/entities Then i have created my entity

#[macro_use] extern crate rocket;

#[get("/tasks")]
async fn get_tasks(db: &State<DatabaseConnection>) -> Json<Vec<String>> {
    let db = db as &DatabaseConnection;
    
    let tasks = Task::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.title)
        .collect::<Vec<String>>();

    Json(tasks)
}

#[post("/tasks", format="json", data="<new_task>")]
async fn create_task(db: &State<DatabaseConnection>, new_task: Json<NewTask>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    println!("{:?}", new_task);

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

#[get("/")]
fn index(_connection: &State<DatabaseConnection>) -> &str {
    "Hello, World"
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![index, get_tasks, create_task])
}