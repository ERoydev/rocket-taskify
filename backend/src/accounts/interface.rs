use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub organization_name: String
}

#[derive(Deserialize, Debug)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}