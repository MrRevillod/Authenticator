
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

// The struct for the login data
#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

// struct for public user data, 
// be used for registration
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmPassword: String
}

// Struct for storing the expired and refresh tokens
#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub token: String,
    pub user_id: ObjectId
}
