pub mod setup;
use std::time::Duration;

pub use setup::set_up_db;
use rocket_taskify::api::task::*; // API Endpoints
use tokio;
use tokio::time::sleep;
use rocket::{Rocket, Orbit};
use rocket::fairing::{Fairing, Info, Kind};

/// SeaORM COMMAND INSTRUCTIONS -------------------------------------------
// sea-orm-cli migrate generate create_tasks_table => Create migration file 

// sea-orm-cli migrate up -u postgres://postgres:test123@localhost:5432/rocket_taskify => When migration file is ready i apply the migration on my Database 
// migrate down => To reverse the migration
// (Adjust database_url <type_of_db>://<name>:<password>@<port>:<host>/<db_name>)

// sea-orm-cli generate entity -u postgres://postgres:test123@localhost:5432/rocket_taskify -o src/entities => Then create entities of my migration so i can use for working with the data

#[macro_use] extern crate rocket;

#[derive(Clone)]
struct SimpleFairing;

#[rocket::async_trait]
impl Fairing for SimpleFairing {
    fn info(&self) -> Info {
        Info {
            name: "Background Task Fairing",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, _: &Rocket<Orbit>) {
        info_!("🚀 Rocket has lifted off!");

        tokio::spawn(async move {
            loop {
                info_!("🔄 Running background task...");

                // Simulate work (replace with actual DB update)
                update_task_priorities().await;
                
                info_!("✅ Task done! Sleeping for 24 hours...");
                sleep(Duration::from_secs(10)).await; // 24 hours
            }
        });
    }
}

async fn update_task_priorities() {
    info_!("===> Updating priorities in the database...");
    
}


#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![
        get_tasks, 
        create_task, 
        delete_task, 
        get_task_by_id, 
        get_tasks_by_completion_status, 
        update_task,
        complete_task,
        ])
        .attach(SimpleFairing)
}
