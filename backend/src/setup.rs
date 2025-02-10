use sea_orm::*;
use std::env;

// const DATABASE_URL: &str = "postgresql://postgres:test123@localhost:5432/rocket_taskify"; // for local development


pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // I load environment variable from render
    
    let db = Database::connect(&database_url).await?;
    println!("✅ Successfully connected to the database!");

    Ok(db)
}
