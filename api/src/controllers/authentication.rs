
use std::str::FromStr;
use mongodb::Collection;
use bcrypt::{verify, hash};
use tower_cookies::Cookies;
use bson::{doc, oid::ObjectId};
use serde_json::to_value;
use chrono::{Utc, Duration as ChronoDuration};
use axum::{extract::{State, Path}, Json, Extension};

use crate::{
    
    config::state::*, 
    
    services::{
        jwt::{sign_jwt, decode_jwt}, 
        authentication::save_exp_token,
    },
    
    responses::{
        Response,
        HttpResponse, ApiResponse,
    },
    
    models::{
        validations::Validation,
        user::{UserModel, UserProfile},
        authentication::{LoginData, PublicUserData}, 
    }, 

    services::{
        cookies::new_cookie, 
        user::check_user_exists,
        authentication::acc_validation_service, 
    },
};

pub async fn login_controller(cookies: Cookies, 
    State(state): ApiState, Json(body): Json<LoginData>) -> HttpResponse {

    let users: Collection<UserModel> = state.db.collection("users");

    let query = users.find_one(doc! { "email": &body.email}, None)
        .await.map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
    ;

    let user = match query {
        Some(user) => user,
        None => return Err(Response::INVALID_CREDENTIALS)
    };

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

    let session_cookie = new_cookie("token", token, time::Duration::minutes(60));
    let refresh_cookie = new_cookie("refresh", refresh, time::Duration::days(7));

    let _ = cookies.add(session_cookie);
    let _ = cookies.add(refresh_cookie);

    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
    };

    Ok(ApiResponse::DataResponse(
        200, "Sesión iniciada", "user", to_value(profile).unwrap())
    )
}

pub async fn register_controller(State(state): 
    ApiState, Json(body): Json<PublicUserData>) -> HttpResponse {

    body.validate()?;

    let users: Collection<UserModel> = state.db.collection("users");

    let filter = doc! { 
        
        "$or": [
            { "username": &body.username },
            { "email": &body.email }
        ]
    };

    let _ = check_user_exists(&state.db, filter).await?;

    let user = UserModel {
        id: ObjectId::new(),
        name: body.name,
        username: body.username,
        email: body.email,
        password: hash(body.password, 7).unwrap(),
        validated: false,
        tasks: Vec::new(),
    };

    let _ = users.insert_one(&user, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)
    ;

    let exp = Utc::now() + ChronoDuration::days(14);
    let secret = format!("{}{}", &JWT_SECRET.to_string(), &user.validated.to_string());

    let payload = (&user.id.to_hex(), &user.email);

    let validation_token = sign_jwt(payload, &secret, exp)?;
    let url = format!("{}/auth/validate/{}/{}", 
        &CLIENT_ADDR.to_string(), &user.id.to_hex(), &validation_token
    );

    let _ = acc_validation_service(&user.email, &url).await?;

    Ok(Response::REGISTER_SUCCESS)
}

pub async fn validate_account(State(state): ApiState, 
    Path((id, token)): Path<(String, String)>) -> HttpResponse {

    if id.len() != 24 || token.len() < 20 {
        return Err(Response::BAD_REQUEST)
    }

    let users: Collection<UserModel> = state.db.collection("users");
    
    let oid = ObjectId::from_str(&id).unwrap();
    let filter = doc! {"_id": oid};

    let query = users.find_one(filter, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)?
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

pub async fn logout_controller(cookies: Cookies, 
    State(state): ApiState, Extension(token): Extension<String>, 
    Extension(user): Extension<UserModel>) -> HttpResponse {

    let refresh_cookie = cookies.get("refresh").map(|cookie| cookie.value().to_string());

    let refresh_token = match refresh_cookie {
        Some(refresh_token) => refresh_token,
        None => return Err(Response::UNAUTHORIZED)
    };

    let _ = save_exp_token(&token, user.id, &state.db).await?;
    let _ = save_exp_token(&refresh_token, user.id, &state.db).await?;

    let session_cookie = new_cookie("token", "".to_string(), time::Duration::minutes(1));
    let refresh_cookie = new_cookie("refresh", "".to_string(), time::Duration::minutes(1));

    let _ = cookies.remove(session_cookie);
    let _ = cookies.remove(refresh_cookie);

    Ok(Response::LOGOUT_SUCCESS)
}

pub async fn validate_session(
    Extension(user): Extension<UserModel>) -> HttpResponse {
    
    let profile = UserProfile {
        id: user.id.to_hex(),
        name: user.name,
        username: user.username,
        email: user.email,
    };

    Ok(ApiResponse::DataResponse(
        200, "Sesión válida", "user", to_value(profile).unwrap())
    )
}

pub async fn protected() -> HttpResponse {
    Ok(Response::SUCCESS)
}
