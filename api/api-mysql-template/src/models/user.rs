
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSchema {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub validated: SqlxBool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub validated: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmPassword: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqlxBool(pub bool);

impl From<i8> for SqlxBool {
    fn from(value: i8) -> Self {
        SqlxBool(value != 0)
    }
}

impl SqlxBool {
    pub fn as_bool(&self) -> bool {
        self.0
    }
}