
use mongodb::{
    Collection, 
    bson::{doc, oid::ObjectId, Document}
};

use bcrypt::hash;
use tower_cookies::Cookies;
use std::collections::HashMap;
use serde_json::{Value, from_value};
use chrono::{Utc, Duration as ChronoDuration};

use reqwest::Client as HttpClient;
use reqwest::multipart::{Form, Part};

use axum::{
    body::Bytes, extract::{Multipart, State,}, Extension, Json, http::HeaderMap,
};

use crate::{
    
    config::state::*, 

    responses::{
        ApiResponse, HttpResponse, Response
    }, 
    
    models::{
        user::{UserModel, UserProfile}, ToJson
    }, 

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

    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
        profilePicture: user.profilePicture,
    };

    Ok(ApiResponse::DataResponse(
        200, "Tu email se ha actualizado", "user", profile.to_json())
    )
}

pub async fn update_profile_picture(State(state): ApiState, 
    Extension(mut user): Extension<UserModel>, 
    headers: HeaderMap, mut multipart: Multipart) -> HttpResponse {

    while let Some(field) = multipart.next_field().await.unwrap() {

        let condition = field.name().unwrap() != "file" || 
            field.content_type().is_none() ||
            headers.get("content-length").is_none()
        ;

        if condition {
            return Err(Response::BAD_REQUEST)
        }

        let content_length = headers.get("content-length")
            .unwrap().to_str().unwrap().parse::<usize>().unwrap()
        ;

        if content_length > 2_000_000 {
            return Err(Response::INVALID_FILE_SIZE)
        }

        #[allow(non_snake_case)]
        let MIME_TYPES = vec!["image/png", "image/jpeg", "image/jpg", "image/webp"];
        
        let mime_type = field.content_type().unwrap().to_string();

        if !MIME_TYPES.contains(&mime_type.as_str()) {
            return Err(Response::INVALID_MIME_TYPE)
        }

        let file_ext = mime_type.split("/").collect::<Vec<&str>>().pop().unwrap();
        let filename = format!("profile-picture.{}", file_ext);

        let file = field.bytes().await.unwrap_or(Bytes::new());

        if file.len() > 2_000_000 || file.len() == 0{
            return Err(Response::INVALID_FILE_SIZE)
        }

        let form = Form::new()
            .part("file", Part::bytes(file.to_vec())
                .file_name(filename)
                .mime_str(mime_type.as_str())
                .unwrap()
            )
        ;
            
        let client = HttpClient::new();

        let response = client.patch(format!("{}/upload", *STORAGE_SERVICE_URL))
            .header("x-api-key", STORAGE_API_KEY.to_string())
            .multipart(form)
            .send()
            .await
            .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
        ;

        if response.status().as_u16() != 200 {
            return Err(Response::INTERNAL_SERVER_ERROR)
        }

        let data: Value = response.json().await
            .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
        ;
        
        if data.get("url").is_none() {
            return Err(Response::INTERNAL_SERVER_ERROR)
        }

        let url = data.get("url").unwrap().as_str().unwrap().to_string();

        user.profilePicture = url;
        user.save(&state.db).await?;

        let profile = UserProfile {
            id: user.id.to_hex(),
            name: user.name,
            username: user.username,
            email: user.email,
            profilePicture: user.profilePicture,
        };

        return Ok(ApiResponse::DataResponse(
            200, "Foto de perfíl actualizada", "user", profile.to_json())
        )
    }

    Ok(Response::SUCCESS)
}
