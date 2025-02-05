use sea_orm::*;
use rocket::State;
use rocket::get;

mod setup;
use setup::set_up_db;
use rust_rocket_tasker::api::*;

// mod api;
// use api::task;

// mod interfaces;
// use interfaces::new_task::NewTask;

// mod error_responder;
// use error_responder::ErrorResponder;

// mod entities;
// use entities::{prelude::*, task};


// sea-orm-cli migrate generate create_tasks_table i have created my migration file
// sea-orm-cli migrate up -u postgres://postgres:test123@localhost:5432/rocket_taskify Then i migrate all unapplyed migrations
// sea-orm-cli generate entity -u postgres://postgres:test123@localhost:5432/rocket_taskify -o src/entities Then i have created my entity

#[macro_use] extern crate rocket;


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