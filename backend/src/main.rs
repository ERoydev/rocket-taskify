use sea_orm::*;

use rocket::{State, get};

mod setup;
use setup::set_up_db;
use rocket_taskify::api::*; // API Endpoints


/// SeaORM COMMAND INSTRUCTIONS -------------------------------------------
// sea-orm-cli migrate generate create_tasks_table => Create migration file 

// sea-orm-cli migrate up -u postgres://postgres:test123@localhost:5432/rocket_taskify => When migration file is ready i apply the migration on my Database 
// migrate down => To reverse the migration
// (Adjust database_url <type_of_db>://<name>:<password>@<port>:<host>/<db_name>)

// sea-orm-cli generate entity -u postgres://postgres:test123@localhost:5432/rocket_taskify -o src/entities => Then create entities of my migration so i can use for working with the data

#[macro_use] extern crate rocket;


#[get("/")] // Initial request TODO: Remove when frontend is implemented
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