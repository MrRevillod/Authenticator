
use mongodb::{
    Collection, 
    bson::{doc, oid::ObjectId, Document}
};

use bcrypt::hash;
use tower_cookies::Cookies;
use std::collections::HashMap;
use serde_json::{Value, from_value};
use axum::{extract::State, Extension, Json};
use chrono::{Utc, Duration as ChronoDuration};

use crate::{
    
    config::state::*, 
    models::{
        ToJson,
        user::{UserModel, UserProfile}
    }, 
    responses::{ApiResponse, HttpResponse, Response}, 
    services::{
        cookies::new_cookie, 
        jwt::{decode_jwt, sign_jwt},
        user::{check_conflict_fields, update_email_service} 
    },
};

pub async fn validate_account(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>, Extension(token): Extension<String>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    let filter = doc! {"_id": oid};

    let query = users.find_one(filter, None).await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let mut user = match query {
        Some(user) => user,
        None => return Err(Response::RESOURCE_NOT_FOUND)
    };

    let secret = format!("{}{}", &JWT_SECRET.to_string(), &user.validated.to_string());

    if let Err(_) = decode_jwt(&token, &secret) {
        return Err(Response::EXPIRED)
    }

    user.validated = true;
    let _ = user.save(&state.db).await?;

    Ok(Response::VALIDATION_SUCCESS)
}

pub async fn delete_account(cookies: Cookies, State(state): ApiState,
    Extension(oid): Extension<ObjectId>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    let query = users.delete_one(doc! {"_id": oid}, None)
        .await.map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    if query.deleted_count == 0 {
        return Err(Response::RESOURCE_NOT_FOUND);
    }

    let session_cookie = new_cookie("token", "".to_string(), time::Duration::minutes(1));
    let refresh_cookie = new_cookie("refresh", "".to_string(), time::Duration::minutes(1));

    let _ = cookies.remove(session_cookie);
    let _ = cookies.remove(refresh_cookie);

    Ok(Response::SUCCESS)
}

pub async fn update_account(
    State(state): ApiState, Extension(oid): Extension<ObjectId>, 
    Extension(user): Extension<UserModel>, Json(body): Json<Value>) -> HttpResponse {

    let mut doc = Document::new();

    let users: Collection<UserModel> = state.db.collection("users");
    let valid_fields = vec!["id", "name", "username", "email", "password", "confirmPassword"];

    let mut body_map: HashMap<String, String> = from_value(body)
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let _ = check_conflict_fields(&state.db, &body_map).await?;

    if let Some(pwd) = body_map.get("password") {
        let encrypted = hash(pwd, 7).unwrap();
        body_map.insert("password".to_string(), encrypted);
    }
    
    for (key, value) in &body_map {
        
        if !valid_fields.contains(&key.as_str()) {
            return Err(Response::BAD_REQUEST);
        }

        if key == "email" || key == "confirmPassword" {
            continue 
        }

        doc.insert(key, value);
    }

    let update = doc! {"$set": doc};

    let _ = users.update_one(doc! {"_id": oid}, update, None) 
        .await.map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let mut email_updated = false;

    if let Some(email) = body_map.get("email") {

        let payload = (&oid.to_hex(), email);
        let exp = Utc::now() + ChronoDuration::hours(24);
        let secret = format!("{}{}", &JWT_SECRET.to_string(), &user.email);

        let token = sign_jwt(payload, &secret, exp)?;

        let url = format!("{}/account/update-email/{}/{}", 
            CLIENT_ADDR.to_string(), oid.to_hex(), token
        );

        update_email_service(&email, &url).await?;
        email_updated = true;
    }

    let mut response_msg = "Tu perfíl se ha actualizado";

    if email_updated {
        response_msg = "Tu perfíl se ha actualizado, revisa tu email para confirmar el cambio";
    }

    let updated = users.find_one(doc! {"_id": oid}, None).await.unwrap().unwrap();

    let profile = UserProfile {
        id: oid.to_hex(),
        name: updated.name,
        username: updated.username,
        email: updated.email,
        profilePicture: updated.profilePicture,
    };

    Ok(ApiResponse::DataResponse(200, 
        &response_msg, "user", profile.to_json())
    )
}

pub async fn update_email(State(state): ApiState, 
    Extension(oid): Extension<ObjectId>, 
    Extension(token): Extension<String>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    let filter = doc! {"_id": oid};

    let query = users.find_one(filter, None).await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
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

    Ok(Response::EMAIL_UPDATE_SUCCESS)
}


