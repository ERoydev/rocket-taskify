// cargo test -- --nocapture => Allow me to see everything printed during the test run, including println!

#[cfg(test)]
mod tests {
    use migration::cli;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::{routes, Build, Rocket};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase}; // In order to use these i added in my Cargo.toml in sea_orm -D feature ["mock"] REMEMBER THIS!
    use rocket_taskify::entities::task;
    use rocket_taskify::{api::*, NewTask};
    use rocket_taskify::interfaces::task_dto::TaskDTO; // I use this when i Deserialize task::Model because when my API return task::Model i convert it TaskDTO which change due_date property to a string instead of use i32, i64

    // Setup function for creating rocket program in every test case
    fn rocket(db: DatabaseConnection) -> Rocket<Build> {
        rocket::build()
            .manage(db)
            .mount("/", routes![get_tasks, create_task]) // Put your API's here
    }

    // Setup func for mock_data
    fn mock_tasks_setup() -> Vec<task::Model> {
        let mock_tasks = vec![
            task::Model {
                id: 1,
                title: "Walk The Dog".to_string(),
                description: "walk the dog".to_string(),
                priority: "High".to_string(),
                due_date: 1738706299, // 04-02-25 when converted to TaskDTO
                is_completed: false,
            },
            task::Model {
                id: 2,
                title: "Wash The Dishes".to_string(),
                description: "wash the dishes".to_string(),
                priority: "Medium".to_string(),
                due_date: 1838706299, // 07-04-28
                is_completed: false,
            },
        ];

        mock_tasks
    }

    // Setup mock database
    fn mock_db_setup(mock_tasks: Vec<task::Model>) -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![mock_tasks])
            .into_connection()
    }

    #[rocket::async_test]
    async fn test_get_it_should_return_all_tasks() {
        // mock data
        let mock_tasks = mock_tasks_setup();

        // Create a mock database with the prepared data
        let db: DatabaseConnection = mock_db_setup(mock_tasks);
        
        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        // Create client who sends requests
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/tasks").dispatch().await;
        
        // Assert the response is with status 200 first
        assert_eq!(response.status(), Status::Ok, "Expected the response to be with status 200, but its not");
        
        // Deserializing happens here, that's why i use TaskDTO because my API returns TaskDTO instead of task::Model type
        let tasks: Vec<TaskDTO> = response.into_json().await.expect("valid response body");
        
        // Check if there are two tasks 
        assert_eq!(2, tasks.len(), "Expected to have 2 tasks, but its false");
    }
    
    #[rocket::async_test]
    async fn test_get_it_should_return_empty_when_no_tasks() {
        let mock_tasks: Vec<task::Model> = vec![];

        let db: DatabaseConnection = mock_db_setup(mock_tasks);

        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        // Create client who sends requests
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/tasks").dispatch().await;

        // Assert the response is with status 200 first
        assert_eq!(response.status(), Status::Ok, "Expected the response to be with status 200, but its not");

        // Deserializing happens here, that's why i use TaskDTO because my API returns TaskDTO instead of task::Model type
        let tasks: Vec<TaskDTO> = response.into_json().await.expect("valid response body");

        // Ensure that its empty
        assert!(tasks.is_empty(), "Expected tasks to be empty, but it wasn't");
    }
    
    #[rocket::async_test]
    async fn test_get_it_should_return_tasks_with_correct_fields() {
        let mock_tasks = mock_tasks_setup();

        let db: DatabaseConnection = mock_db_setup(mock_tasks);

        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        // Create client who sends requests
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/tasks").dispatch().await;

        // Assert the response is with status 200 first
        assert_eq!(response.status(), Status::Ok, "Expected the response to be with status 200, but its not");

        // Deserializing happens here, that's why i use TaskDTO because my API returns TaskDTO instead of task::Model type
        let tasks: Vec<TaskDTO> = response.into_json().await.expect("valid response body");

        assert_eq!("Walk The Dog", tasks[0].title);
        assert_eq!("walk the dog", tasks[0].description);
        assert_eq!("High", tasks[0].priority);
        assert_eq!("04-02-25", tasks[0].due_date);
        assert!(!tasks[0].is_completed);
        assert_eq!(1738706299, tasks[0].due_date_timestamp);

        assert_eq!("Wash The Dishes", tasks[1].title);
        assert_eq!("wash the dishes", tasks[1].description);
        assert_eq!("Medium", tasks[1].priority);
        assert_eq!("07-04-28", tasks[1].due_date);
        assert!(!tasks[1].is_completed);
        assert_eq!(1838706299, tasks[1].due_date_timestamp);
    }
    

    #[rocket::async_test]
    async fn test_get_it_should_return_error() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors(vec![])
        .into_connection();

        let rocket = rocket(db);

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/tasks").dispatch().await;

        assert_eq!(response.status(), Status::InternalServerError);

        let body = response.into_string().await.expect("valid response body");
        
        assert_eq!("Database error: Unable to complete the operation. Please try again later".to_string(), body);
    }


    #[rocket::async_test]
    async fn test_post_it_should_create_one_task() {
        let mock_tasks = mock_tasks_setup();

        // Create a mock database with the prepared data
        let db: DatabaseConnection = mock_db_setup(mock_tasks);
        
        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        let new_task = NewTask {
            title: "title".to_string(),
            description: "description".to_string(),
            priority: "High".to_string(),
            due_date: 1738706299,   
            is_completed: false
        };

        // // Create client who sends requests
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let post_response = client.post("/tasks").json(&new_task).dispatch().await;

        assert_eq!(post_response.status(), Status::Ok, "Expected the response to be with status 200, but its not");


        println!("{:?}", post_response);
        // let task: TaskDTO = post_response.into_json().await.expect("valid response body");

        // println!("{:?}", task);
        
        // let get_response = client.get("/tasks").dispatch().await;

        // println!("{:?}", get_response);
    }


}
