use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::entities::revoked_tokens::{Entity as RevokedToken, Column};

pub async fn is_token_revoked(db: &DatabaseConnection, token: &str) -> bool {
    RevokedToken::find()
        .filter(Column::Token.eq(token)) // ✅ Now .eq() will work correctly
        .one(db)
        .await
        .unwrap_or(None) // If there's an error, return None instead of crashing
        .is_some() // ✅ Returns `true` if the token exists (revoked), `false` otherwise
}