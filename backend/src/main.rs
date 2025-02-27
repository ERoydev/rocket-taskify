pub mod setup;

use std::time::Duration;
use reqwest::Client as ReqClient;
use rocket::http::{Method, Status};
pub use setup::set_up_db;
use rocket_taskify::api::task::*; // API Endpoints
use rocket_taskify::api::auth::*;

use tokio;
use tokio::time::sleep;

use rocket::{Rocket, Orbit};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

/// SeaORM COMMAND INSTRUCTIONS -------------------------------------------
// sea-orm-cli migrate generate create_tasks_table => Create migration file 

// sea-orm-cli migrate up -u postgres://postgres:test123@localhost:5432/rocket_taskify => When migration file is ready i apply the migration on my Database 
// migrate down => To reverse the migration make new changes and migrate up to upload them
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

                update_task_priorities().await;
                
                info_!("✅ Task done! Sleeping for 24 hours...");
                sleep(Duration::from_secs(24 * 3600)).await; // 24 hours
            }
        });
    }
}


#[derive(Default)]
struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_raw_header("Access-Control-Allow-Origin", "*");
        response.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
        response.set_raw_header("Access-Control-Allow-Headers", "Content-Type");

        // ✅ When i try to DELETE ruquest from frontend it is sended as OPTIONS instead of DELETE
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
    }
}

async fn update_task_priorities() {
    info_!("===> Updating priorities in the database...");
    
    let client = ReqClient::new();

    let api_url = "http://localhost:8000/tasks/update_priority"; // ================================================= API_URL FOR UPDATING PRIORITIES =================

    let response = client.post(api_url)
        .send()
        .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    info_!("Successfully updated tasks:");
                } else {
                    error_!("Failed to update tasks, status: {}", res.status());
                }
            }
            Err(err) => {
                error_!("Error making request: {}", err);
            }
        }
}


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
            ])
        .attach(SimpleFairing)
        .attach(Cors)
}
