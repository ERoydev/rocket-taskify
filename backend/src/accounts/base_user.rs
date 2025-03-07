use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use rocket::{http::Status, serde::json::Json};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher as ArgonPasswordHasher, PasswordVerifier, SaltString
    }, Algorithm, Argon2, Params, ParamsBuilder, Version
};
use chrono::{self, Utc};
use rocket::State;

use crate::{entities::user::{self, ActiveModel, Model}, ErrorResponder};
use super::{interface::UserCredentials, jwt::{create_jwt, JwtResponse}};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseUser {
    id: i32,
    email: String,
    password: String,

    // Role-based flags
    // is_admin type of flag is going to be handled with Many To Many relationship to Roles
    is_active: bool,     
    
    // Meta data
    created_at: String,
    updated_at: String,
    last_login: Option<String>, // Default is Null
}

impl BaseUser {
    fn new(user: Model) -> BaseUser {
        BaseUser {
            id: user.id,
            email: user.email,
            password: user.password,
            is_active: user.is_active,
            last_login: user.last_login,
            created_at: user.created_at,
            updated_at: user.updated_at
        }
    }
}

// Implementation for user created from entities after db migrations
// I prefer keeping this code in this module because i am going to use defined functions in this module ONLY
impl user::ActiveModel {

    fn new(email: String, hashed_password: String) -> user::ActiveModel {
        let now = Utc::now().naive_utc().to_string(); 

        user::ActiveModel {
            email: Set(email.clone()),
            password: Set(hashed_password), // You should hash this before storing
            is_active: Set(false),
            last_login: Set(None), // Optional field
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
            ..Default::default() // Default handles the auto-incrementing ID
        }
    }
}

// BASE USER MANAGER

pub trait BaseUserManager {
    fn create_user(db: &State<DatabaseConnection>, email: String, password: String) -> impl std::future::Future<Output = Result<BaseUser, ErrorResponder>>; // Returns Future which means i should .await() this function
    fn login_user(db: &rocket::State<sea_orm::DatabaseConnection>, user: Json<UserCredentials>) -> impl std::future::Future<Output = Result<JwtResponse, ErrorResponder>>;
    fn check_if_user_exists(db: &rocket::State<sea_orm::DatabaseConnection>, email: &str) -> impl std::future::Future<Output = Result<bool, ErrorResponder>>;
    fn get_user_by_email(db: &rocket::State<sea_orm::DatabaseConnection>, email: &str) -> impl std::future::Future<Output = Result<user::Model, ErrorResponder>>;
    fn verify_password(raw_password: &str, hashed_password: &str) -> Result<(), ErrorResponder>;
    // fn create_admin_user(id: i32, email: String, password: String) -> Self;
}


impl BaseUserManager for BaseUser {
    async fn create_user(db: &rocket::State<sea_orm::DatabaseConnection>, email: String, password: String) -> Result<BaseUser, ErrorResponder>  {
        let now: String = Utc::now().naive_utc().to_string();
        
        let hashed_password = match BaseUser::encrypt_password(password) {
            Ok(hash) => hash, // If success, store the hash
            Err(err) => {
                return Err(ErrorResponder::from(err))
            }
        };

        let user_exist = BaseUser::check_if_user_exists(db, &email).await?;

        if user_exist {
            return Err(ErrorResponder::from("User with this email already exists!"));
        } 

        let new_user: ActiveModel = user::ActiveModel::new(email, hashed_password);

        let inserted_user: Model = new_user.insert(db.inner()).await.expect("Failed to insert user"); // Insert into database

        let base_user: BaseUser = BaseUser::new(inserted_user);

        Ok(base_user)
    }

    async fn login_user(db: &rocket::State<sea_orm::DatabaseConnection>, user_credentials: Json<UserCredentials>) -> Result<JwtResponse, ErrorResponder> {
        let user_exist = BaseUser::check_if_user_exists(db, &user_credentials.email).await?;

        if !user_exist {
            return Err(ErrorResponder::from("User with this email does not exists!"));
        } 

        let user = BaseUser::get_user_by_email(&db, &user_credentials.email).await?;
        BaseUser::verify_password(&user_credentials.password, &user.password)?;
        
        // JWT TOKEN LOGIC BELLOW
        match create_jwt(user.id) {
            Ok(jwt_response) => Ok(jwt_response),
            Err(_) => Err(ErrorResponder::new("Unable to create JWT token", Status::InternalServerError)),
        }   
    }

    async fn check_if_user_exists(db: &rocket::State<sea_orm::DatabaseConnection>, email: &str) -> Result<bool, ErrorResponder> {
        
        let result: Result<Option<Model>, DbErr> = user::Entity::find()
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
            Err(err) => Err(ErrorResponder::from(err))
        }
    }

    async fn get_user_by_email(db: &rocket::State<sea_orm::DatabaseConnection>, email: &str) -> Result<user::Model, ErrorResponder> {

        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db.inner())
            .await;

        match user {
            Ok(Some(model)) => Ok(model),
            Ok(None) => Err(ErrorResponder::from("User with this email address is not found")),
            Err(err) => Err(ErrorResponder::from(err)),
        }
    }

    fn verify_password(entered_password: &str, stored_hash: &str) -> Result<(), ErrorResponder> {
        let argon2 = Argon2::default();
    
        // Try parsing the stored hash
        let parsed_hash = PasswordHash::new(stored_hash)
            .map_err(|_| ErrorResponder::new("Invalid stored password format", Status::InternalServerError))?;
    
        // Verify password and return error if incorrect
        argon2
            .verify_password(entered_password.as_bytes(), &parsed_hash)
            .map_err(|_| ErrorResponder::new("Invalid credentials", Status::Unauthorized))
    }
}


// HASH
pub trait HashManager {
    fn encrypt_password(password: String) -> Result<String, ErrorResponder>; // Returns encrypted password
}

impl HashManager for BaseUser {
    fn encrypt_password(password: String) -> Result<String, ErrorResponder> {
        let salt: SaltString = SaltString::generate(&mut OsRng);
        let argon2: Argon2<'_> = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hashed| hashed.to_string())
            .map_err(|_| ErrorResponder::from("Error with hashing password") ) // Handle errors safely
    }
}
