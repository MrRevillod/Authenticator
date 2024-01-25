
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use jsonwebtoken::{
    decode, 
    encode, 
    Header, 
    Validation,
    DecodingKey, 
    EncodingKey, 
};

use crate::responses::{
    ApiResult, 
    error::{
        UNAUTHORIZED,
        INTERNAL_SERVER_ERROR
    }
};

#[derive(Serialize, Deserialize)]
pub struct JwtPayload {
    pub id: String,
    pub exp: usize,
}

pub fn sign_jwt(user_id: &String, secret: &String, exp: DateTime<Utc>) -> ApiResult<String> {
    
    let payload = JwtPayload {
        id: user_id.to_string(),
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(INTERNAL_SERVER_ERROR),
    }
}

pub fn decode_jwt(token: &String, secret: &String) -> ApiResult<String> {

    let payload = decode::<JwtPayload>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match payload {
        
        Ok(payload) => {
            
            let payload = payload.claims;
            
            let exp = payload.exp as i64;
            let now = Utc::now().timestamp();

            if exp < now {
                return Err(UNAUTHORIZED)
            }

            Ok(payload.id)
        }
        
        Err(_) => Err(UNAUTHORIZED),
    }
}