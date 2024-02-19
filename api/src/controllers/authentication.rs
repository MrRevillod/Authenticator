
use serde_json::Value;
use bcrypt::{verify, hash};
use tower_cookies::Cookies;
use std::collections::HashMap;
use chrono::{Utc, Duration as ChronoDuration};

use mongodb::{
    Collection, bson::{doc, oid::ObjectId}
};

use axum::{
    extract::{Path, State}, Extension, Json
};

use crate::{
    
    config::state::*, 

    models::{
        authentication::{LoginData, RegisterData}, 
        user::{UserModel, UserProfile}, validations::Validation, ToJson 
    }, 
    responses::{
        ApiResponse, HttpResponse, Response
    }, 
    services::{
        cookies::new_cookie, 
        jwt::{decode_jwt, save_exp_token, sign_jwt}, 
        microservices::{send_acc_validation_email, send_recovery_email}, 
        user::{check_conflict_fields, find_user, oid_from_str}
    }
};

pub async fn login_controller(cookies: Cookies, 
    State(state): ApiState, Json(body): Json<LoginData>) -> HttpResponse {

    let filter = doc! { "email": &body.email };
    let user = find_user(&state.db, filter).await?;

    if !verify(&body.password, &user.password).unwrap() {
        return Err(Response::INVALID_CREDENTIALS)
    }

    if !user.validated {
        return Err(Response::ACCOUNT_NOT_VALIDATED)
    }

    let session_exp = Utc::now() + ChronoDuration::minutes(60);
    let refresh_exp = Utc::now() + ChronoDuration::days(7);

    let payload = (&user.id.to_hex(), &user.email);
    
    let token = sign_jwt(payload.clone(), &JWT_SECRET, session_exp)?;
    let refresh = sign_jwt(payload, &JWT_SECRET, refresh_exp)?;

    let session_cookie = new_cookie("token", Some(&token), "SESSION");
    let refresh_cookie = new_cookie("refresh", Some(&refresh), "REFRESH");

    cookies.add(session_cookie);
    cookies.add(refresh_cookie);

    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
        profilePicture: user.profilePicture,
    };

    Ok(ApiResponse::DataResponse(
        200, "Sesión iniciada", "user", profile.to_json())
    )
}

pub async fn register_controller(State(state): 
    ApiState, Json(body): Json<RegisterData>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");
    body.validate()?;

    let mut conflict_map = HashMap::new();

    conflict_map.insert("username".to_string(), body.username.clone());
    conflict_map.insert("email".to_string(), body.email.clone());

    check_conflict_fields(&state.db, &conflict_map).await?;

    let user = UserModel {
        id: ObjectId::new(),
        name: body.name,
        username: body.username,
        email: body.email,
        password: hash(body.password, 7).unwrap(),
        validated: false,
        profilePicture: DEFAULT_PROFILE_PICTURE.to_string()
    };

    users.insert_one(&user, None).await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let exp = Utc::now() + ChronoDuration::days(14);
    let secret = format!("{}{}", *JWT_SECRET, user.validated);

    let payload = (&user.id.to_hex(), &user.email);
    let validation_token = sign_jwt(payload, &secret, exp)?;
    
    let url = format!("{}/account/validate/{}/{}", 
        *CLIENT_ADDR, user.id.to_hex(), validation_token
    );

    send_acc_validation_email(&user.email, &url).await?;

    Ok(Response::REGISTER_SUCCESS)
}

pub async fn logout_controller(cookies: Cookies, 
    State(state): ApiState, Extension(token): Extension<String>, 
    Extension(user): Extension<UserModel>) -> HttpResponse {

    let refresh_cookie = cookies.get("refresh")
        .map(|cookie| cookie.value().to_string())
    ;

    let refresh_token = match refresh_cookie {
        Some(refresh_token) => refresh_token,
        None => return Err(Response::UNAUTHORIZED)
    };

    save_exp_token(&state.db, &token, user.id).await?;
    save_exp_token(&state.db, &refresh_token, user.id).await?;

    let session_cookie = new_cookie("token", None, "SESSION");
    let refresh_cookie = new_cookie("refresh", None, "REFRESH");

    cookies.remove(session_cookie);
    cookies.remove(refresh_cookie);

    Ok(Response::LOGOUT_SUCCESS)
}

pub async fn validate_session(
    Extension(user): Extension<UserModel>) -> HttpResponse {
    
    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
        profilePicture: user.profilePicture,
    };

    Ok(ApiResponse::DataResponse(
        200, "Sesión válida", "user", profile.to_json())
    )
}

pub async fn send_reset_password_email(State(state): ApiState, 
    Json(body): Json<Value>) -> HttpResponse {

    if !body.is_object() || body.get("email").is_none() {
        return Err(Response::BAD_REQUEST)
    }

    let email = body.get("email").unwrap().as_str().unwrap();
    let user = find_user(&state.db, doc! { "email": email }).await?;

    let payload = (&user.id.to_hex(), &user.email);
    let exp = Utc::now() + ChronoDuration::days(7);

    let secret = format!("{}{}", JWT_SECRET.to_string(), &user.password);
    let token = sign_jwt(payload, &secret, exp)?;

    let recovery_url = format!("{}/auth/reset-password/{}/{}", 
        *CLIENT_ADDR, user.id.to_hex(), token
    );

    send_recovery_email(&user.email, &recovery_url).await?;

    Ok(Response::PASSWORD_RESET_REQUEST)
}

pub async fn reset_password_validation(State(state): ApiState,
    Path((id, token)): Path<(String, String)>) -> HttpResponse {

    let oid = oid_from_str(&id)?;
    let user = find_user(&state.db, doc! { "_id": oid }).await?;
    
    let secret = format!("{}{}", *JWT_SECRET, user.password);
    decode_jwt(&token, &secret)?;
    
    Ok(Response::SUCCESS)
}

pub async fn reset_password(State(state): ApiState,
    Path((id, token)): Path<(String, String)>, Json(body): Json<Value>) -> HttpResponse {

    let condition = !body.is_object() || 
        body.get("password").is_none() || 
        body.get("confirmPassword").is_none()
    ;

    if condition {
        return Err(Response::BAD_REQUEST)
    }

    let password = body.get("password").unwrap().as_str().unwrap();
    let confirm_pwd = body.get("confirmPassword").unwrap().as_str().unwrap();
    let password_mismatch = password != confirm_pwd;

    if password_mismatch {
        return Err(Response::BAD_REQUEST)
    }

    let oid = oid_from_str(&id)?;
    let mut user = find_user(&state.db, doc! { "_id": oid }).await?;

    let secret = format!("{}{}", *JWT_SECRET, &user.password);
    
    decode_jwt(&token, &secret)?;

    let new_password = hash(password, 7).unwrap();

    user.password = new_password;
    user.save(&state.db).await?;

    Ok(Response::PASSWORD_RESET_SUCCESS)
}
