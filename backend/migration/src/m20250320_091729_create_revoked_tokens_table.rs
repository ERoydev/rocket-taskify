use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RevokedTokens::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RevokedTokens::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                    )
                    .col(ColumnDef::new(RevokedTokens::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(RevokedTokens::UserId).integer().not_null())
                    .col(ColumnDef::new(RevokedTokens::ExpiresAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RevokedTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RevokedTokens {
    Table,
    Id,
    Token,
    UserId,
    ExpiresAt,
}
