use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::IsActive).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::LastLogin).string()) // I allow this to be Null when user haven't logged in
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null().string())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null().string())
                    .to_owned(),
                )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Password,
    IsActive,
    LastLogin,
    CreatedAt,
    UpdatedAt,
}
