
use mongodb::{
    Collection, 
    Database, 
    bson::Document
};
use reqwest::Body;
use serde_json::json;

use crate::{
    models::user::UserModel, 
    responses::{ApiResult, Response},
    config::state::{MAILER_API_KEY, MAILER_SERVICE_URL},
};

pub async fn check_user_exists(
    db: &Database, filter: Document) -> ApiResult<bool> {

    let users: Collection<UserModel> = db.collection("users");

    let query = users.find_one(filter, None).await;

    match query {
        
        Ok(Some(user)) => {
            
            dbg!(&user);
            println!("{}", user.name);
            return Err(Response::USER_ALREADY_EXISTS)
        }
        
        Ok(None) => {
            return Ok(false)
        }
        
        Err(_) => {
            return Err(Response::INTERNAL_SERVER_ERROR)
        }
    }
}

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