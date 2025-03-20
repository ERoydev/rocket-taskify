pub use sea_orm_migration::prelude::*;

mod m20250204_170857_create_tasks_table;
mod m20250227_221505_create_user_table;
mod m20250301_011213_create_profile_table;
mod m20250320_091729_create_revoked_tokens_table;





pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // I add migrations that i want to apply by hand here ----------------
            Box::new(m20250204_170857_create_tasks_table::Migration),
            Box::new(m20250227_221505_create_user_table::Migration),
            Box::new(m20250301_011213_create_profile_table::Migration),
            Box::new(m20250320_091729_create_revoked_tokens_table::Migration),
        ]
    }
}
