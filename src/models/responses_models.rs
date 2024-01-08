
use serde::{Serialize, Deserialize};
use super::user_models::UserSchema;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccess {
    pub message: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterSuccess {
    pub message: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserSuccess {
    pub user: UserSchema,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUsersSuccess {
    pub users: Vec<UserSchema>,
    pub results: usize,
}