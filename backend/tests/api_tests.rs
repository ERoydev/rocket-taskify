// cargo test -- --nocapture => Allow me to see everything printed during the test run, including println!


#[cfg(test)]
mod tests {
    use rocket::local::blocking::Client;
    use rocket::routes;
    use rocket_taskify;
    use rocket_taskify::rocket;
    use rocket_taskify::api::get_tasks;
    use rocket_taskify::setup::set_up_db;

    async fn establish_test_connection() -> sea_orm::DatabaseConnection {
        let db = match set_up_db().await {
            Ok(db) => db,
            Err(err) => panic!("{}", err),
        };
        db
    }

    fn rocket_with_db() -> rocket::Rocket<rocket::Build> {
        let db = tokio::runtime::Runtime::new().unwrap().block_on(establish_test_connection());

        let rocket = rocket::build().manage(db).mount("/", routes![get_tasks]); // Mount routes here

        rocket
    }

    #[test]
    fn it_should_return_all_tasks() {
        let rocket = rocket_with_db();
    
        let client = Client::tracked(rocket).unwrap();
    
        let req = client.get("/tasks");
    
        let response = req.dispatch();

        println!("{:?}", response);
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
