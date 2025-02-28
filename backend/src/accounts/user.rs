
// Used for authentication API
use serde::{Deserialize, Serialize};

use super::base_user::{BaseUser};



#[derive(Deserialize, Serialize, Debug)]
pub struct Customer {
    pub base: BaseUser,
    pub first_name: String,
    pub last_name: String,
}

// impl Customer {
//     fn new(id: i32, email: String, password: String, first_name: String, last_name: String) -> Customer {
//         let base = BaseUser::create_user(id, email, password);

//         Customer {
//             base,
//             first_name,
//             last_name
//         }
//     }
// }

    // fn hash_password(&self) -> String {
    //     /*
    //     Used Argon2 to hash the password -> Considered the most secure current algorithm for that job
    //     1. Generate a Random Salt -> unique random value (22bytes) used to prevent hash collisions.
    //     */

    //     let salt = SaltString::generate(&mut OsRng);
    // }
// }



// impl User {
//     pub fn new(id: i32, name: String, password: String) -> User {
//         Self::hash_password_argon2(&password);

//         User{
//             id,
//             name,
//             password
//         }
//     }

//     fn hash_password_argon2(password: &str)  {
//         /*
//         Used Argon2 to hash the password -> Considered the most secure current algorithm for that job
//             1. Generate a Random Salt -> unique random value (22bytes) used to prevent hash collisions.
//          */

//         let salt = SaltString::generate(&mut OsRng);

//         let configuration = Params::new(
//             Params::D
//         )

        // let argon2 = Argon2::default(); // Uses Argon2id the hybrid version

    