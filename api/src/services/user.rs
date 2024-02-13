
use std::collections::HashMap;

use mongodb::{
    Collection, 
    Database, 
    bson::doc
};

use reqwest::Body;
use serde_json::{json, to_value};

use crate::{
    models::user::UserModel, 
    responses::{ApiResult, Response, ApiResponse},
    config::state::{MAILER_API_KEY, MAILER_SERVICE_URL},
};

pub async fn update_email_service(email: &String, url: &String) -> ApiResult<()> {

    let client = reqwest::Client::new();
    let body = json!({ "email": email, "url": url });
    
    let body_bytes = serde_json::to_vec(&body)
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    let response = client
        .post(format!("{}/email-change", MAILER_SERVICE_URL.to_string()))
        .header("Content-Type", "application/json")
        .header("x-api-key", MAILER_API_KEY.to_string())
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

pub async fn check_conflict_fields(db: &Database, body_map: &HashMap<String, String>) -> ApiResult<()> {

    let users: Collection<UserModel> = db.collection("users");

    let mut existing_fields = HashMap::new();

    if body_map.get("email").is_some() {
        
        let email_exists = users.find_one(doc! {"email": &body_map["email"]}, None).await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
            .is_some()
        ;
        
        if email_exists {
            existing_fields.insert("email", "Ya existe un usuario con este email");
        }
    }

    if body_map.get("username").is_some() {
        
        let username_exists = users.find_one(doc! {"username": &body_map["username"]}, None).await
            .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
            .is_some()
        ;
        
        if username_exists {
            existing_fields.insert("username", "Ya existe un usuario con este alias");
        }
    }

    if !existing_fields.is_empty() {
        return Err(ApiResponse::DataResponse(
            409, "Conflicto", "conflicts", to_value(existing_fields).unwrap())
        )
    }

    Ok(())
}