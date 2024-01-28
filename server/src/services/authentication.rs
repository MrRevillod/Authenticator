
use chrono::{Duration, Utc};
use mongodb::{
    Collection,
    Database,
};

use reqwest::Body;
use bson::{doc, oid::ObjectId};
use serde_json::{json, to_vec};

use crate::{
    
    responses::{
        Response,
        ApiResult,
    },
    
    models::authentication::Token,
};

use super::jwt::{decode_jwt, sign_jwt};

pub async fn save_token(token_type: &str, db: &Database, 
    token: &String, user_id: ObjectId) -> ApiResult<()> {

    let tokens: Collection<Token> = db.collection(token_type);

    let token = Token {
        id: ObjectId::new(),
        token: token.clone(),
        user_id
    };

    let _ = tokens.insert_one(&token, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    Ok(())
}

pub async fn is_token(token_type: &str, db: &Database, token: &String) -> ApiResult<bool> {

    let tokens: Collection<Token> = db.collection(token_type);

    let query = tokens.find_one(doc! { "token": token }, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    match query {
        Some(_) => return Err(Response::UNAUTHORIZED),
        None => return Ok(false)
    }
}

pub async fn acc_validation_service(key: &String, 
    service_url: &String, email: &String, url: &String) -> ApiResult<()> {

    let client = reqwest::Client::new();
    let body = json!({ "email": email, "url": url });
    let body_bytes = to_vec(&body).map_err(|_| return Response::INTERNAL_SERVER_ERROR)?;

    let response = client
        .post(service_url)
        .header("Content-Type", "application/json")
        .header("x-api-key", key)
        .body(Body::from(body_bytes))
        .send()
        .await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    match response.status().as_u16() {
        200 => (),
        400 => return Err(Response::BAD_REQUEST),
        _ => return Err(Response::INTERNAL_SERVER_ERROR)
    }

    Ok(())
}


pub async fn new_session_token(refresh_token: &String, db: &Database, secret: &String) -> ApiResult<String> {

    let exp_tokens: Collection<Token> = db.collection("exp_tokens");

    let query = exp_tokens.find_one(doc! { "token": refresh_token }, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    if let Some(_) = query {
        return Err(Response::UNAUTHORIZED)
    }

    let user_id = decode_jwt(refresh_token, secret)?;

    let exp = Utc::now() + Duration::minutes(1);

    let new_token = sign_jwt(&user_id, secret, exp)?;

    Ok(new_token)
}
