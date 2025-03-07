use sea_orm::{sqlx::types::uuid::Error, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher as ArgonPasswordHasher, PasswordVerifier, SaltString
    }, Algorithm, Argon2, Params, ParamsBuilder, Version
};
use chrono::{self, Utc};
use rocket::State;

use crate::{entities::user, ErrorResponder};
use super::interface::UserCredentials;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseUser {
    id: i32,
    email: String,
    password: String,

    // Role-based flags
    // is_admin type of flag is going to be handled with Many To Many relationship to Roles
    is_active: bool,     
    last_login: Option<String>, // Default is Null

    // Meta data
    created_at: String,
    updated_at: String,
}

// BASE USER MANAGER

pub trait BaseUserManager {
    fn create_user(db: &State<DatabaseConnection>, email: String, password: String) -> impl std::future::Future<Output = Result<BaseUser, ErrorResponder>>; // Returns Future which means i should .await() this function
    fn login_user(db: &rocket::State<sea_orm::DatabaseConnection>, user: Json<UserCredentials>) -> Result<(), ErrorResponder>;
    fn check_if_user_exists(db: &rocket::State<sea_orm::DatabaseConnection>, email: &str) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>>;
    // fn create_admin_user(id: i32, email: String, password: String) -> Self;
}

impl BaseUserManager for BaseUser {
    async fn create_user(db: &rocket::State<sea_orm::DatabaseConnection>, email: String, password: String) -> Result<BaseUser, ErrorResponder>  {
        let now = Utc::now().naive_utc().to_string();
        
        let hashed_password = match BaseUser::encrypt_password(password) {
            Ok(hash) => hash, // If success, store the hash
            Err(err) => {
                return Err(ErrorResponder::from(err))
            }
        };

        let user_exist = BaseUser::check_if_user_exists(db, &email).await?;

        if user_exist {
            return Err(ErrorResponder::from("User already exists"));
        } 

        let new_user = user::ActiveModel {
            email: Set(email.clone()),
            password: Set(hashed_password), // You should hash this before storing
            is_active: Set(false),
            last_login: Set(None), // Optional field
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
            ..Default::default() // Default handles the auto-incrementing ID
        };

        let inserted_user = new_user.insert(db.inner()).await.expect("Failed to insert user"); // Insert into database

        let base_user = BaseUser {
            id: inserted_user.id,
            email: inserted_user.email,
            password: inserted_user.password,
            is_active: inserted_user.is_active,
            last_login: inserted_user.last_login,
            created_at: inserted_user.created_at,
            updated_at: inserted_user.updated_at,
        };

        Ok(base_user)
    }

    fn login_user(db: &rocket::State<sea_orm::DatabaseConnection>, user: Json<UserCredentials>) -> Result<(), ErrorResponder> {
        Ok(())
    }

    async fn check_if_user_exists(db: &rocket::State<sea_orm::DatabaseConnection>, email: &str) -> Result<bool, sea_orm::DbErr> {
        
        let result = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db.inner())
        .await;

        match result {
            Ok(Some(value)) => {
                // User with this email already exist.
                Ok(true)
            },
            Ok(None) => {
                // User with this email does not exist.
                Ok(false)
            },
            // Unexpected error happened when querying the db
            Err(err) => Err(err)
        }
    }
}



// HASH
pub trait HashManager {
    fn encrypt_password(password: String) -> Result<String, ErrorResponder>; // Returns encrypted password
}

impl HashManager for BaseUser {
    fn encrypt_password(password: String) -> Result<String, ErrorResponder> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hashed| hashed.to_string())
            .map_err(|_| ErrorResponder::from("Error with hashing password") ) // Handle errors safely
    }
}
