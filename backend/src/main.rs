#[macro_use] extern crate rocket;

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
    rocket::build().mount("/", routes![index])
}