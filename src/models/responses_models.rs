
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSuccessSchema {
    pub message: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterSuccessSchema {
    pub message: String,
    pub url: String,
}