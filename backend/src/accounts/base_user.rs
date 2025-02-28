use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher as ArgonPasswordHasher, PasswordVerifier, SaltString
    }, Algorithm, Argon2, Params, ParamsBuilder, Version
};
use chrono::{self, NaiveDate, NaiveDateTime, Utc};
use rocket::State;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseUser {
    id: i32,
    email: String,
    password: String,

    // Role-based flags
    is_admin: bool,          // Determines if the user is an administrator
    is_active: bool,         // Determines if the user is active
    is_verified: bool,       // Determines if the email is verified
    is_suspended: bool,      // Determines if the user is suspended

    // Meta data
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

// pub trait BaseUserManager {
//     fn encrypt_password(&self) -> String; // Returns encrypted password
//     fn create_user(db: &State<DatabaseConnection>, email: String, password: String) -> Self;
//     // fn create_admin_user(id: i32, email: String, password: String) -> Self;
// }

// impl BaseUserManager for BaseUser {
//     fn create_user(db: &State<DatabaseConnection>, email: String, password: String) -> Self {
//         let db = db as &DatabaseConnection;

//         let now = Utc::now().naive_utc();

//         let mut base_user = BaseUser {
//             email,
//             password,
//             is_admin: false,
//             is_active: false,
//             is_verified: false,
//             is_suspended: false,
//             created_at: Set(now),
//             updated_at: Set(now)
//         };

//         let hashed_password = base_user.encrypt_password();
//         base_user.password = hashed_password;

//         base_user
//     }

//     fn encrypt_password(&self) -> String {
//         // Generate random salt (22bytes) to prevent hash collisions
//         let salt = SaltString::generate(&mut OsRng);

//         // I can use Argon2 Struct to create the hashing algorithm using custom configuration values if i want to optimize memory usage of this hashing operation
//         // - Default uses the Hybrid Argon algorithm that is optimized to work for the both security vulnerabilities (GPU cracking attacks,  side-channel attacks)
//         let argon2 = Argon2::default(); 

//         let password_bytes = self.password.as_bytes(); // Convert password to bytes

//         argon2.hash_password(password_bytes, &salt).unwrap().to_string()
//     }
// }

