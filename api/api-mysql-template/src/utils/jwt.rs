
use axum::http::HeaderValue;
use chrono::{DateTime, Utc};

use jsonwebtoken::{
    encode,
    decode,
    Header,
    Validation,
    EncodingKey,
    DecodingKey,
};

use crate::models::authentication::*;
use crate::responses::ApiResult;
use crate::responses::error::ApiError;


pub fn sign_jwt(uuid: &String, secret: &String, exp: DateTime<Utc>) -> ApiResult<String> {

    let payload = JwtPayload {
        uuid: uuid.to_string(),
        exp: exp.timestamp() as usize
    };

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes())
    );

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(ApiError::UnexpectedError)
    }
}

pub fn decode_jwt(token: &String, secret: &String) -> ApiResult<String> {
    
    let token_data = decode::<JwtPayload>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims.uuid),
        Err(_) => Err(ApiError::Unauthorized)
    }
}

pub fn split_authorization(authorization: Option<&HeaderValue>) -> ApiResult<String> {

    if let Some(authorization) = authorization {
        
        let authorization = authorization.to_str().unwrap();
        let token = authorization.split(" ").collect::<Vec<&str>>();
        
        if token.len() == 2 {
            return Ok(token[1].to_string())
        }
    }

    Err(ApiError::Unauthorized)
}
