
// Used for authentication API
use serde::{Deserialize, Serialize};

use crate::entities::user;

use super::{base_user::BaseUser, interface::NewUser};



#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub base: BaseUser,
    pub first_name: String,
    pub last_name: String,
    pub organization_name: String,
}

impl User {
    pub fn create_customer_user(user_data: NewUser, base: BaseUser) -> User {
        User {
            base,
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            organization_name: user_data.organization_name,
        }
    }
}
