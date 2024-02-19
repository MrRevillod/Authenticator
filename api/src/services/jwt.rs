
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use mongodb::{Database, bson::{doc, oid::ObjectId}, Collection};

use jsonwebtoken::{
    decode, encode, 
    Header, Validation,
    DecodingKey, EncodingKey, 
};

use crate::{
    models::authentication::Token,
    responses::{
        ApiResult, 
        error::{ UNAUTHORIZED, INTERNAL_SERVER_ERROR}
    },
    config::state::JWT_SECRET,
};

#[derive(Serialize, Deserialize)]
pub struct JwtPayload {
    pub id: String,
    pub email: String,
    pub exp: usize,
}

pub fn sign_jwt((id, email): (&String, &String), 
    secret: &String, exp: DateTime<Utc>) -> ApiResult<String> {
    
    let payload = JwtPayload {
        id: id.to_string(),
        email: email.to_string(),
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

pub fn decode_jwt(token: &String, secret: &String) -> ApiResult<JwtPayload> {

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

            Ok(payload)
        }
        
        Err(_) => Err(UNAUTHORIZED),
    }
}

pub async fn check_token_exp(db: &Database, token: &String) -> ApiResult<()> {

    let exp_tokens: Collection<Token> = db.collection("exp_tokens");

    let query = exp_tokens.find_one(doc! { "token": token }, None)
        .await.map_err(|_| INTERNAL_SERVER_ERROR)?
    ;

    match query {
        Some(_) => return Err(UNAUTHORIZED),
        None => return Ok(())
    }
}

/// Returns a new session token with the session/user id of the request 

pub async fn new_session_token(db: &Database, refresh_token: &String) -> ApiResult<(String, String)> {

    check_token_exp(db, refresh_token).await?;

    let payload = decode_jwt(refresh_token, &JWT_SECRET)?;
    let exp = Utc::now() + ChronoDuration::minutes(60);

    let new_payload = (&payload.id, &payload.email);
    let new_token = sign_jwt(new_payload, &JWT_SECRET, exp)?;

    Ok((new_token, payload.id))
}

/// Saves the token in the database to check for expiration

pub async fn save_exp_token(db: &Database, 
    token: &String, user_id: ObjectId) -> ApiResult<()> {

    let tokens: Collection<Token> = db.collection("exp_tokens");

    let token = Token {
        id: ObjectId::new(),
        token: token.clone(),
        user_id
    };

    let _ = tokens.insert_one(&token, None).await
        .map_err(|_| INTERNAL_SERVER_ERROR)?
    ;

    Ok(())
}