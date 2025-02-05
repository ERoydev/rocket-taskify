// cargo test -- --nocapture => Allow me to see everything printed during the test run, including println!


#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::{routes, Build, Rocket};
    use sea_orm::{DatabaseBackend, DatabaseConnection, EntityTrait, MockDatabase}; // In order to use these i added in my Cargo.toml in sea_orm -D feature ["mock"] REMEMBER THIS!
    use rocket_taskify::entities::task;
    use rocket_taskify::api::*;
    use rocket_taskify::interfaces::task_dto::TaskDTO; // I use this when i Deserialize task::Model because when my API return task::Model i convert it TaskDTO which change due_date property to a string instead of use i32, i64

    // Setup function for creating rocket program in every test case
    fn rocket(db: DatabaseConnection) -> Rocket<Build> {
        rocket::build()
            .manage(db)
            .mount("/", routes![get_tasks]) // Put your API's here
    }

    #[rocket::async_test]
    async fn it_should_return_all_tasks() {
        // mock data
        let mock_tasks = vec![
            task::Model {
                id: 1,
                title: "Task 1".to_string(),
                description: "Description 1".to_string(),
                priority: "High".to_string(),
                due_date: 1738706299,
                is_completed: false,
            },
            task::Model {
                id: 2,
                title: "Task 2".to_string(),
                description: "Description 2".to_string(),
                priority: "Medium".to_string(),
                due_date: 1738706399,
                is_completed: false,
            },
        ];
        

        // Create a mock database with the prepared data
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![mock_tasks.clone()])
            .into_connection();
        
        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        // Create client who sends requests
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/tasks").dispatch().await;
        
        // Assert the response is with status 200 first
        assert_eq!(response.status(), Status::Ok);
        
        // Deserializing happens here, that's why i use TaskDTO because my API returns TaskDTO instead of task::Model type
        let tasks: Vec<TaskDTO> = response.into_json().await.expect("valid response body");
        
        // Check if there are two tasks 
        assert_eq!(2, tasks.len());
    }
    
    #[test]
    fn it_should_return_empty_when_no_tasks() {
        
    }
    
    #[test]
    fn it_should_return_tasks_with_correct_fields() {
    
    }
    
    #[test]
    fn it_should_return_error_when_fetching_tasks_fails() {
    
    }

}
