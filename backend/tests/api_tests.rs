// cargo test -- --nocapture => Allow me to see everything printed during the test run, including println!


#[cfg(test)]
mod tests {
    use chrono::Local;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::{routes, Build, Rocket};
    use rocket_taskify::api::task::*;
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase}; // In order to use these i added in my Cargo.toml in sea_orm -D feature ["mock"] REMEMBER THIS!
    use rocket_taskify::entities::task;
    use rocket_taskify::NewTask;
    use rocket_taskify::interfaces::task_dto::TaskDTO;
    use serde::Serialize;
    use sea_orm::*;

    // Setup function for creating rocket program in every test case
    fn rocket(db: DatabaseConnection) -> Rocket<Build> {
        rocket::build()
            .manage(db)
            .mount("/", routes![get_tasks, create_task, delete_task, update_tasks_priority]) // Put your API's here
    }

    // Setup func for mock_data
    fn mock_tasks_setup() -> Vec<task::Model> {
        let mock_tasks = vec![
            task::Model {
                id: 1,
                title: "Walk The Dog".to_string(),
                description: "walk the dog".to_string(),
                priority: "high".to_string(),
                due_date: 1738706299, // 04-02-25 when converted to TaskDTO
                is_completed: false,
                is_critical: false,
            },
            task::Model {
                id: 2,
                title: "Wash The Dishes".to_string(),
                description: "wash the dishes".to_string(),
                priority: "medium".to_string(),
                due_date: 1838706299, // 07-04-28
                is_completed: false,
                is_critical: false,
            },
            task::Model {
                id: 3,
                title: "Workout".to_string(),
                description: "workout".to_string(),
                priority: "low".to_string(),
                due_date: 1838706299, // 07-04-28
                is_completed: false,
                is_critical: false,
            },
            task::Model {
                id: 4,
                title: "Completed".to_string(),
                description: "completed".to_string(),
                priority: "low".to_string(),
                due_date: 1838706299, // 07-04-28
                is_completed: true,
                is_critical: false,
            },
        ];

        mock_tasks
    }

    // Setup mock database
    fn mock_db_setup(mock_tasks: Vec<task::Model>) -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![mock_tasks])
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 2,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 3,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 4,
                    rows_affected: 1,
                },
            ])
            .into_connection()
    }

    #[rocket::async_test]
    async fn test_get_tasks_it_should_return_all_tasks() {
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
        assert_eq!(4, tasks.len(), "Expected to have 2 tasks, but its false");
    }
    
    #[rocket::async_test]
    async fn test_get_tasks_it_should_return_empty_when_no_tasks() {
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
    async fn test_get_tasks_it_should_return_tasks_with_correct_fields() {
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
        assert_eq!("high", tasks[0].priority);
        assert_eq!("04-02-25", tasks[0].due_date);
        assert!(!tasks[0].is_completed);
        assert_eq!(1738706299, tasks[0].due_date_timestamp);

        assert_eq!("Wash The Dishes", tasks[1].title);
        assert_eq!("wash the dishes", tasks[1].description);
        assert_eq!("medium", tasks[1].priority);
        assert_eq!("07-04-28", tasks[1].due_date);
        assert!(!tasks[1].is_completed);
        assert_eq!(1838706299, tasks[1].due_date_timestamp);
    }
    

    #[rocket::async_test]
    async fn test_get_tasks_it_should_return_database_error() {
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
    async fn test_post_tasks_it_should_create_task_successfully() {
        let mock_tasks = mock_tasks_setup();

        // Create a mock database with the prepared data
        let db: DatabaseConnection = mock_db_setup(mock_tasks);

        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        let new_task = NewTask {
            title: "title".to_string(),
            description: "description".to_string(),
            due_date: 1738706299,   
            is_completed: false,
            is_critical: false,
        };

        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        
        let post_response = client.post("/tasks").json(&new_task).dispatch().await;

        assert_eq!(post_response.status(), Status::Ok, "Expected the response to be with status 200, but its not");
    }

    #[rocket::async_test]
    async fn test_post_tasks_it_should_return_database_error() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors(vec![])
        .into_connection();

        let rocket = rocket(db);

        let new_task = NewTask {
            title: "title".to_string(),
            description: "description".to_string(),
            due_date: 1738706299,   
            is_completed: false,
            is_critical: false,
        };

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.post("/tasks").json(&new_task).dispatch().await;

        assert_eq!(response.status(), Status::InternalServerError);

        let body = response.into_string().await.expect("valid response body");
        
        assert_eq!("Database error: Unable to complete the operation. Please try again later".to_string(), body);
    }

    #[rocket::async_test]
    async fn test_post_tasks_tasks_with_wrong_entity_type_returns_unprocessable_entity_error() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors(vec![])
        .into_connection();

        let rocket = rocket(db);

        #[derive(Serialize)]
        struct ErrorTaskEntity {
            pub title: i32,
            pub description: i32,
            pub priority: i32,
            pub due_date: i32,
            pub is_completed: bool,
        }

        let new_task = ErrorTaskEntity {
            title: 1231,
            description: 232,
            priority: 12312,
            due_date: 1738706299,   
            is_completed: false
        };

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.post("/tasks").json(&new_task).dispatch().await;

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[rocket::async_test]
    async fn test_post_tasks_with_wrong_data_types_in_entity_returns_unprocessable_entity_error() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors(vec![])
        .into_connection();

        let rocket = rocket(db);

        #[derive(Serialize)]
        struct NewTask {
            pub title: String,
            pub description: String,
            pub priority: String,
            pub due_date: i32, // i use UNIX-Timestamp
            pub is_completed: String,
        }

        let new_task = NewTask {
            title: "asdasd".to_string(),
            description: "asda".to_string(),
            priority: "High".to_string(),
            due_date: 1738706299,   
            is_completed: "true".to_string()
        };

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.post("/tasks").json(&new_task).dispatch().await;

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[rocket::async_test]
    async fn test_post_tasks_check_returned_fields_should_be_valid() {
        let mock_tasks = mock_tasks_setup();

        // Create a mock database with the prepared data
        let db: DatabaseConnection = mock_db_setup(mock_tasks);

        // Build the Rocket instance with the mocked database
        let rocket = rocket(db);

        let new_task = NewTask {
            title: "title".to_string(),
            description: "description".to_string(),
            due_date: 1738706299,   
            is_completed: false,
            is_critical: false,
        };

        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        
        let post_response = client.post("/tasks").json(&new_task).dispatch().await;

        assert_eq!(post_response.status(), Status::Ok, "Expected the response to be with status 200, but its not");

        let task: TaskDTO = post_response.into_json().await.expect("valid response body");

        assert_eq!(4, task.id);
        assert_eq!("title".to_string(), task.title);
        assert_eq!("description".to_string(), task.description);
        assert_eq!("expired".to_string(), task.priority);
        assert_eq!("04-02-25".to_string(), task.due_date);
        assert_eq!(false, task.is_completed);
        assert_eq!(1738706299, task.due_date_timestamp);
    }

    #[rocket::async_test]
    async fn it_should_delete_task_by_id() {
        let mock_tasks = mock_tasks_setup();
        let db: DatabaseConnection = mock_db_setup(mock_tasks);
        let rocket = rocket(db);
    
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.delete("/tasks/1").dispatch().await;
  
        assert_eq!(response.status(), rocket::http::Status::Ok);
        
    }


    #[rocket::async_test]
    async fn it_should_verify_task_is_deleted() {
        let mock_tasks = mock_tasks_setup();
        let db: DatabaseConnection = mock_db_setup(mock_tasks);
        let rocket = rocket(db);

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        // Perform a DELETE request for task with ID = 1
        let response = client.delete("/tasks/1").dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::Ok);

        let respone_json: String = response.into_json().await.expect("Valid json body");

        assert_eq!("Task successfully deleted.", respone_json);
    }
    
    #[rocket::async_test]
    async fn test_update_priority() {
        let mut mock_tasks = vec![
            task::Model {
                id: 1,
                title: "Walk The Dog".to_string(),
                description: "walk the dog".to_string(),
                priority: "high".to_string(),
                due_date: 1738706299, // 04-02-25 when converted to TaskDTO
                is_completed: false,
                is_critical: false,
            },
            task::Model {
                id: 2,
                title: "Wash The Dishes".to_string(),
                description: "wash the dishes".to_string(),
                priority: "medium".to_string(),
                due_date: 1838706299, // 07-04-28
                is_completed: false,
                is_critical: false,
            },
        ];

        let now_timestamp: i64 = Local::now().timestamp();

        mock_tasks.iter_mut().for_each(|task| {
            task.due_date = now_timestamp + 5;
        });

        let db = mock_db_setup(mock_tasks);

        let rocket = rocket(db);
    
        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.post("/tasks/update_priority").dispatch().await;

        println!("---------------------TIMESTAMP: {:?}", response);
    }
}


