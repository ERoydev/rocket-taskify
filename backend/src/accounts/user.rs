
// Used for authentication API

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDTO { // Data-Transfer-Object
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    password: String, // Hashed password using Argon2
}

impl User {
    pub fn new(id: i32, name: String, password: String) -> User {
        Self::hash_password_argon2(&password);

        User{
            id,
            name,
            password
        }
    }

    fn hash_password_argon2(_password: &str)  {
        /*
        Used Argon2 to hash the password -> Considered the most secury current algorithm for that job
            1. Generate a Random Salt -> unique random value (16bytes) used to prevent hash collisions.
         */

        let salt = SaltString::generate(&mut OsRng);

        println!("SALTED ARGON2: {}", salt);
    }
    
}