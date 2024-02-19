
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JwtPayload {
    pub uuid: String,
    pub exp: usize,
}