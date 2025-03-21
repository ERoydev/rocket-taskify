pub mod setup;

use rocket_taskify::tasks::CleanupTask;
pub use setup::set_up_db;


use rocket_taskify::cors::Cors;
use rocket_taskify::api::task::*; // API Endpoints
use rocket_taskify::api::auth::*;

use tokio;

/// SeaORM COMMAND INSTRUCTIONS -------------------------------------------
// sea-orm-cli migrate generate create_tasks_table => Create migration file 

// sea-orm-cli migrate up -u postgres://postgres:test123@localhost:5432/rocket_taskify => When migration file is ready i apply the migration on my Database 
// migrate down => To reverse the migration make new changes and migrate up to upload them
// (Adjust database_url <type_of_db>://<name>:<password>@<port>:<host>/<db_name>)

// sea-orm-cli generate entity -u postgres://postgres:test123@localhost:5432/rocket_taskify -o src/entities => Then create entities of my migration so i can use for working with the data

#[macro_use] extern crate rocket;



#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/",          
            routes![
                get_tasks, 
                create_task, 
                delete_task, 
                get_task_by_id, 
                get_tasks_by_completion_status, 
                update_task,
                complete_task,
                critical_task,
                update_tasks_priority,
                signup,
                login,
                logout,
                cleanup_expired_tokens,
            ])
        .attach(CleanupTask)
        .attach(Cors)
}
