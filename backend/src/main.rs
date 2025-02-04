#[macro_use] extern crate rocket;
use std::env;
mod models;
use models::task_model::{Task, Priority};

#[get("/")]
fn index() -> String {
    let task = Task {
        id: 1,
        title: "Walk the dog".to_string(),
        description: "asdad".to_string(),
        priority: Priority::High,
        due_date: 123123,
        is_completed: true,
    };

    task.title
}

#[launch]
fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![index, bakeries])
}