
use std::str::FromStr;

use bson::{
    doc, 
    oid::ObjectId
};

use serde_json::to_value;
use tower_cookies::Cookies;
use cookie::Cookie;

use time::Duration as CookieDuration;

use mongodb::Collection;
use axum::{extract::{State, Path}, Json, Extension};

use bcrypt::{verify, hash};
use chrono::{Utc, Duration as ChronoDuration};

use crate::{
    
    services::{
        authentication::save_token,
        jwt::{sign_jwt, decode_jwt}, 
    },
    
    models::user::UserModel,
    config::state::ApiState, 
    
    responses::{
        Response,
        HttpResponse, ApiResponse,
    },
    
    models::{
        validations::Validation,
        authentication::{LoginData, PublicUserData}, 
    }, 

    services::{user::check_user_exists, authentication::is_token},
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

    let exp = Utc::now() + ChronoDuration::minutes(2);
    let token = sign_jwt(&user.id.to_hex(), &state.jwt_secret, exp)?;

    let mut cookie = Cookie::new("token", token);

    cookie.set_http_only(true); // non js accessible
    cookie.set_secure(false); // true => https only / false => http
    cookie.set_path("/");
    cookie.set_max_age(CookieDuration::minutes(2));

    let _ = cookies.add(cookie);

    Ok(ApiResponse::DataResponse(
        200, "Sesión iniciada", "exp", to_value(exp.timestamp()).unwrap())
    )
}

pub async fn register_controller(State(state): 
    ApiState, Json(body): Json<PublicUserData>) -> HttpResponse {

    body.validate()?;

    let users: Collection<UserModel> = state.db.collection("users");

    let filter = doc! { 
        "username": &body.username, 
        "email": &body.email 
    };

    let _ = check_user_exists(&state.db, filter).await?;

    let user = UserModel {
        id: ObjectId::new(),
        name: body.name,
        username: body.username,
        email: body.email,
        password: hash(body.password, 7).unwrap(),
        validated: true, // false in production
        tasks: Vec::new(),
    };

    let _ = users.insert_one(&user, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)
    ;

    // here the call to email microservice

    Ok(Response::REGISTER_SUCCESS)
}

pub async fn validate_account(State(state): ApiState, 
    Path((id, token)): Path<(String, String)>) -> HttpResponse {

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

    let secret = format!("{}{}", &state.jwt_secret, &user.validated.to_string());

    if let Err(_) = decode_jwt(&token, &secret) {
        return Err(Response::EXPIRED)
    }

    user.validated = true;

    let _ = user.save(&state.db).await?;

    Ok(Response::VALIDATION_SUCCESS)
}

pub async fn logout_controller(
    cookies: Cookies, 
    State(state): ApiState, 
    Extension(token): Extension<String>, 
    Extension(user): Extension<UserModel>) -> HttpResponse {

    let _ = save_token("exp_tokens", &state.db, &token, user.id).await?;

    let mut cookie = Cookie::from("token");

    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookie.set_path("/");
    cookie.set_max_age(CookieDuration::minutes(5));

    let _ = cookies.remove(cookie);

    Ok(Response::LOGOUT_SUCCESS)
}

pub async fn refresh_token(
    cookies: Cookies, 
    State(state): ApiState, 
    Extension(token): Extension<String>, 
    Extension(user): Extension<UserModel>) -> HttpResponse {

    let _ = is_token("refresh_tokens", &state.db, &token).await?;
    let _ = save_token("exp_tokens", &state.db, &token, user.id).await?;

    let exp = Utc::now() + ChronoDuration::days(1);
    let new_token = sign_jwt(&user.id.to_hex(), &state.jwt_secret, exp)?;

    let _ = save_token("refresh_tokens", &state.db, &new_token, user.id).await?;

    let mut cookie = Cookie::new("token", new_token);

    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookie.set_path("/");
    cookie.set_max_age(CookieDuration::minutes(2));

    let _ = cookies.add(cookie);

    Ok(ApiResponse::DataResponse(200, "Exito", "exp", to_value(exp.timestamp()).unwrap()))
}

pub async fn validate_session() -> HttpResponse {
    Ok(Response::SUCCESS)
}
