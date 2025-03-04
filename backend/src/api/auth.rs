use rocket::{post, get};
use rocket::{serde::json::Json, State};

use sea_orm::DatabaseConnection;

use crate::accounts::{interface::{NewUser, UserCredentials}, base_user::{BaseUser, BaseUserManager}, users::User};
use crate::ErrorResponder;


#[post("/auth/signup", format="json", data="<user_data>")]
pub async fn signup(db: &State<DatabaseConnection>, user_data: Json<NewUser>) -> Result<Json<User>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let base_user = BaseUser::create_user(db.into(), user_data.email.clone(), user_data.password.clone()).await;
    let user = User::create_customer_user(user_data.into_inner(), base_user);
    // TODO -> I need to fix email collisions, creating Profile with the User data 
    
    Ok(Json(user))
}

#[post("/auth/login", format="json", data="<user_data>")]
pub async fn login(db: &State<DatabaseConnection>, user_data: Json<UserCredentials>) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    Ok(())
}

#[post("/auth/logout/<id>")]
pub async fn logout(db: &State<DatabaseConnection>, id: i32) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    Ok(())
}
