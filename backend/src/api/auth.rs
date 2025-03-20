use chrono::NaiveDateTime;
use rocket::http::{ContentType, Status};
use rocket::{get, post, Response};
use rocket::{serde::json::Json, State};

use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use crate::accounts::auth_responder::AuthResponder;
use crate::accounts::middlewares::AuthenticatedUser;
use crate::accounts::{base_user, jwt};
use crate::accounts::{interface::{NewUser, UserCredentials}, base_user::{BaseUser, BaseUserManager}, users::User};
use crate::error_responder::ErrorResponder;
use crate::entities::{revoked_tokens, revoked_tokens::Entity as RevokedToken};

#[post("/auth/signup", format="json", data="<user_data>")]
pub async fn signup(db: &State<DatabaseConnection>, user_data: Json<NewUser>) -> Result<Json<User>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let base_user = BaseUser::create_user(db.into(), user_data.email.clone(), user_data.password.clone()).await;

    match base_user {
        Ok(base_user) => {
            let user = User::create_customer_user(user_data.into_inner(), base_user);
            // TODO -> I need to fix email collisions and to create a Profile with the User data 
            
            Ok(Json(user))
        }
        Err(err) => {
            return Err(err);
        }
    }

}

#[post("/auth/login", format="json", data="<user_data>")]
pub async fn login(db: &State<DatabaseConnection>, user_data: Json<UserCredentials>) -> Result<AuthResponder, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let jwt_response = BaseUser::login_user(db.into(), user_data).await?; // All the logic happens here

    Ok(AuthResponder {
        token: jwt_response.token,
        status: Status::Ok,
        expires_in: jwt_response.exp,
    })

}

/*
I use revoked_token table 
*/

#[post("/auth/logout")]
pub async fn logout(db: &State<DatabaseConnection>, user: Result<AuthenticatedUser, ErrorResponder>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = user?;

    println!("AUTHENTICEDUSER: {:?}", user);

    let expires_at = NaiveDateTime::from_timestamp_opt(user.claims.exp as i64, 0)
        .ok_or_else(|| ErrorResponder::new("Invalid token expiration", Status::BadRequest))?;

    let revoked_token_data = revoked_tokens::ActiveModel {
        id: NotSet,
        token: ActiveValue::Set(user.token),
        user_id: ActiveValue::Set(user.claims.subject_id),
        expires_at: ActiveValue::Set(expires_at)
    };

    revoked_tokens::Entity::insert(revoked_token_data)
        .exec(db)
        .await
        .map_err(|_| ErrorResponder::new("Failed to revoke token", Status::InternalServerError))?;


    Ok(())
}
