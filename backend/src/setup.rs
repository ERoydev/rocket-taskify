use sea_orm::*;
use std::env;
use dotenv::dotenv;

/*
DATABASE URL IS IN THIS FORMAT

<type of db>://<db_username>:<db_password>@<url>:<port>/<db_name>

Example:
    postgres://postgres:parolatami123@localhost:5432/rocket_taskify_db
*/

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables!");

    let db = Database::connect(database_url).await?;
    println!("✅ Successfully connected to the database!");

    Ok(db)
}
