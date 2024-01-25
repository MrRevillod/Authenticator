
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PublicUserData {
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