
use std::collections::HashMap;

use bcrypt::hash;
use mongodb::Collection;
use bson::{doc, oid::ObjectId, Document};
use chrono::{Utc, Duration as ChronoDuration};
use serde_json::{from_value, to_value, Value};
use axum::{extract::{Path, State}, Extension, Json};

use crate::{
    config::state::*, 
    models::user::{UserModel, UserProfile}, 
    responses::{ApiResponse, HttpResponse, Response}, 
    services::{jwt::{decode_jwt, sign_jwt}, user::update_email_service}
};

pub async fn get_user(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");

    let query = users.find_one(doc! {"_id": oid}, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    let user = match query {
        Some(user) => user,
        None => return Err(Response::RESOURCE_NOT_FOUND)
    };

    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
    };

    Ok(ApiResponse::DataResponse(
        200, "success", "user", to_value(profile).unwrap()
    ))
}

pub async fn delete_account(State(state): ApiState,
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");

    let query = users.delete_one(doc! {"_id": oid}, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    if query.deleted_count == 0 {
        return Err(Response::RESOURCE_NOT_FOUND);
    }

    Ok(Response::SUCCESS)
}

pub async fn update_profile(
    State(state): ApiState, Extension(oid): Extension<ObjectId>, 
    Extension(user): Extension<UserModel>, Json(body): Json<Value>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    let valid_fields = vec!["id", "name", "username", "email", "password"];

    let mut body_map: HashMap<String, String> = from_value(body)
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

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
        return Err(ApiResponse::BadRequest(existing_fields))
    }

    if let Some(pwd) = body_map.get("password") {
        let encrypted = hash(pwd, 7).unwrap();
        body_map.insert("password".to_string(), encrypted);
    }
    
    let mut doc = Document::new();

    for (key, value) in &body_map {
        
        if !valid_fields.contains(&key.as_str()) {
            return Err(Response::BAD_REQUEST);
        }

        if key == "email" {
            continue 
        }

        doc.insert(key, value);
    }

    let update = doc! {"$set": doc};

    let _ = users.update_one(doc! {"_id": oid}, update, None) 
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    if let Some(email) = body_map.get("email") {

        let payload = (&oid.to_hex(), email);
        let exp = Utc::now() + ChronoDuration::hours(24);
        let secret = format!("{}{}", &JWT_SECRET.to_string(), &user.email);

        let token = sign_jwt(payload, &secret, exp)?;

        let url = format!("{}/users/change-email/{}/{}", 
            CLIENT_ADDR.to_string(), oid.to_hex(), token
        );

        update_email_service(&email, &url).await?;
    }

    let updated = users.find_one(doc! {"_id": oid}, None).await.unwrap().unwrap();

    let profile = UserProfile {
        id: oid.to_hex(),
        name: updated.name,
        username: updated.username,
        email: updated.email,
    };

    Ok(ApiResponse::DataResponse(200, "success", "profile updated", to_value(profile).unwrap()))
}

pub async fn update_email(State(state): ApiState, 
    Path(token): Path<String>, Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");

    let filter = doc! {"_id": oid};

    let query = users.find_one(filter, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    let mut user = match query {
        Some(user) => user,
        None => return Err(Response::RESOURCE_NOT_FOUND)
    };

    let secret = format!("{}{}", JWT_SECRET.to_string(), &user.email);    
    let payload = decode_jwt(&token, &secret)?;

    if user.id.to_hex() != oid.to_hex() {
        return Err(Response::UNAUTHORIZED)
    }

    user.email = payload.email;
    user.save(&state.db).await?;

    Ok(Response::SUCCESS)
}
