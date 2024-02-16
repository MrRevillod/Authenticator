
use mongodb::{
    Database,
    Collection,
    bson::{doc, oid::ObjectId}
};

use reqwest::Body;
use chrono::{Duration, Utc};
use serde_json::{json, to_vec};

use crate::{
    
    responses::{
        Response,
        ApiResult,
    },
    
    config::state::*,
    models::authentication::Token,
};

use super::jwt::{decode_jwt, sign_jwt};

pub async fn save_exp_token(token: &String, 
    user_id: ObjectId, db: &Database) -> ApiResult<()> {

    let tokens: Collection<Token> = db.collection("exp_tokens");

    let token = Token {
        id: ObjectId::new(),
        token: token.clone(),
        user_id
    };

    let _ = tokens.insert_one(&token, None).await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    Ok(())
}

pub async fn is_exp_token(token: &String, db: &Database) -> ApiResult<bool> {

    let tokens: Collection<Token> = db.collection("exp_tokens");

    let query = tokens.find_one(doc! { "token": token }, None)
        .await.map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    match query {
        Some(_) => return Err(Response::UNAUTHORIZED),
        None => return Ok(false)
    }
}

pub async fn acc_validation_service(email: &String, url: &String) -> ApiResult<()> {

    let client = reqwest::Client::new();
    let body = json!({ "email": email, "url": url });
    let body_bytes = to_vec(&body)
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let url = format!("{}/email-verification", MAILER_SERVICE_URL.to_string());

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("x-api-key", MAILER_API_KEY.to_string())
        .body(Body::from(body_bytes))
        .send()
        .await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    match response.status().as_u16() {
        200 => (),
        400 => return Err(Response::BAD_REQUEST),
        _ => return Err(Response::INTERNAL_SERVER_ERROR)
    }

    Ok(())
}

pub async fn new_session_token(refresh_token: &String, db: &Database) -> ApiResult<String> {

    let exp_tokens: Collection<Token> = db.collection("exp_tokens");

    let query = exp_tokens.find_one(doc! { "token": refresh_token }, None)
        .await.map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    if let Some(_) = query {
        return Err(Response::UNAUTHORIZED)
    }

    let payload = decode_jwt(refresh_token, &JWT_SECRET)?;
    let exp = Utc::now() + Duration::minutes(60);

    let new_payload = (&payload.id, &payload.email);
    let new_token = sign_jwt(new_payload, &JWT_SECRET, exp)?;

    Ok(new_token)
}
