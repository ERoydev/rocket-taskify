use sea_orm_migration::prelude::*;

/*
Use the following documentation when creating Migration files:
    https://github.com/SeaQL/sea-query#table-create
*/


#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
