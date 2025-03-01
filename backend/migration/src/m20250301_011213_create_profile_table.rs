use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Profile::UserId)
                        .integer()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(Profile::FirstName).string())
                    .col(ColumnDef::new(Profile::LastName).string())
                    .col(ColumnDef::new(Profile::OrganizationName).string())
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_profile_user") // Custom name <Optional>
                        .from(Profile::Table, Profile::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade) // ONCASCADE Delete profile when user is deleted
                        .on_update(ForeignKeyAction::Cascade), // ONCASCADE for update
                    )
                    .to_owned(),
                )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Profile {
    Table,
    UserId, // I do not create Id for profile i use UserId as such
    FirstName,
    LastName,
    OrganizationName
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}