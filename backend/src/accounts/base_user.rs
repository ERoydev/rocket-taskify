use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher as ArgonPasswordHasher, PasswordVerifier, SaltString
    }, Algorithm, Argon2, Params, ParamsBuilder, Version
};
use chrono::{self, Utc};
use rocket::State;

use crate::entities::user;

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


pub trait BaseUserManager {
    fn encrypt_password(password: String) -> String; // Returns encrypted password
    fn create_user(db: &State<DatabaseConnection>, email: String, password: String) -> impl std::future::Future<Output = BaseUser>; // Returns Future which means i should .await() this function
    // fn create_admin_user(id: i32, email: String, password: String) -> Self;
}

impl BaseUserManager for BaseUser {
    async fn create_user(db: &rocket::State<sea_orm::DatabaseConnection>, email: String, password: String) -> Self  {
        let now = Utc::now().naive_utc().to_string();

        let hashed_password = BaseUser::encrypt_password(password);

        let new_user = user::ActiveModel {
            email: Set(email.clone()),
            password: Set(hashed_password.clone()), // You should hash this before storing
            is_active: Set(false),
            last_login: Set(None), // Optional field
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
            ..Default::default() // Default handles the auto-incrementing ID
        };

        let inserted_user = new_user.insert(db.inner()).await.expect("Failed to insert user");

        let base_user = BaseUser {
            id: inserted_user.id,
            email: inserted_user.email,
            password: inserted_user.password,
            is_active: inserted_user.is_active,
            last_login: inserted_user.last_login,
            created_at: inserted_user.created_at,
            updated_at: inserted_user.updated_at,
        };

        base_user

    }

    fn encrypt_password(password: String) -> String {
        // Generate random salt (22bytes) to prevent hash collisions
        let salt = SaltString::generate(&mut OsRng);

        // I can use Argon2 Struct to create the hashing algorithm using custom configuration values if i want to optimize memory usage of this hashing operation
        // - Default uses the Hybrid Argon algorithm that is optimized to work for the both security vulnerabilities (GPU cracking attacks,  side-channel attacks)
        let argon2 = Argon2::default(); 

        let password_bytes = password.as_bytes(); // Convert password to bytes

        argon2.hash_password(password_bytes, &salt).unwrap().to_string()
    }
}

