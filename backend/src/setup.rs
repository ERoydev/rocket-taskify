use sea_orm::*;

const DATABASE_URL: &str = "postgresql://postgres:test123@localhost:5432/rocket_taskify";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    println!("✅ Successfully connected to the database!");

    Ok(db)
}
