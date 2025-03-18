use rocket::http::{ContentType, Status};
use rocket::{get, post, Response};
use rocket::{serde::json::Json, State};

use sea_orm::DatabaseConnection;

use crate::accounts::auth_responder::AuthResponder;
use crate::accounts::middlewares::AuthenticatedUser;
use crate::accounts::{base_user, jwt};
use crate::accounts::{interface::{NewUser, UserCredentials}, base_user::{BaseUser, BaseUserManager}, users::User};
use crate::error_responder::ErrorResponder;

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

#[post("/auth/logout")]
pub async fn logout(db: &State<DatabaseConnection>, user: Result<AuthenticatedUser, ErrorResponder>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let user = user?;

    Ok(())
}
