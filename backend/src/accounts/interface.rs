use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct UserDTO { // Data-Transfer-Object
    pub id: i32,
    pub email: String,
    pub password: String,
}
