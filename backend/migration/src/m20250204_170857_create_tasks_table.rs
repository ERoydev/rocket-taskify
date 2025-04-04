use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Task::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Task::Title).string().not_null())
                    .col(ColumnDef::new(Task::Description).string().not_null())
                    .col(ColumnDef::new(Task::Priority).string().not_null())
                    .col(ColumnDef::new(Task::DueDate).big_integer().not_null())
                    .col(ColumnDef::new(Task::IsCompleted).boolean().not_null().default(false))
                    .col(ColumnDef::new(Task::IsCritical).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Title,
    Description,
    Priority,
    DueDate,
    IsCompleted,
    IsCritical
}
