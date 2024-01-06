
use jsonwebtoken::{
    encode,
    decode,
    Header,
    Validation,
    EncodingKey,
    DecodingKey,
};

use super::types::{ApiResult, ApiError};
use crate::models::auth_models::*;

pub async fn sign_jwt(uuid: &String, secret: &String) -> ApiResult<String> {

    let exp = chrono::Utc::now() + chrono::Duration::days(1);

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

pub async fn decode_jwt(token: &String, secret: &String) -> ApiResult<String> {
    
    let token_data = decode::<JwtPayload>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims.uuid),
        Err(_) => Err(ApiError::InvalidToken)
    }
}