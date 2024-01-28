
use std::str::FromStr;

use bson::{
    doc, 
    oid::ObjectId
};

use serde_json::to_value;
use tower_cookies::Cookies;

use mongodb::Collection;
use axum::{extract::{State, Path}, Json, Extension};

use bcrypt::{verify, hash};
use chrono::{Utc, Duration as ChronoDuration};

use crate::{
    
    services::{
        authentication::save_token,
        jwt::{sign_jwt, decode_jwt}, 
    },
    
    models::user::{UserModel, UserProfile},
    config::state::ApiState, 
    
    responses::{
        Response,
        HttpResponse, ApiResponse,
    },
    
    models::{
        validations::Validation,
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

    let session_exp = Utc::now() + ChronoDuration::minutes(1);
    let refresh_exp = Utc::now() + ChronoDuration::minutes(2);
    
    let token = sign_jwt(&user.id.to_hex(), &state.jwt_secret, session_exp)?;
    let refresh = sign_jwt(&user.id.to_hex(), &state.jwt_secret, refresh_exp)?;

    let session_cookie = new_cookie("token", token, time::Duration::minutes(1));
    let refresh_cookie = new_cookie("refresh", refresh, time::Duration::minutes(2));

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
        validated: false, // false in production
        tasks: Vec::new(),
    };

    let _ = users.insert_one(&user, None).await
        .map_err(|_| return Response::INTERNAL_SERVER_ERROR)
    ;

    let exp = Utc::now() + ChronoDuration::days(1);
    let secret = format!("{}{}", &state.jwt_secret, &user.validated.to_string());

    let validation_token = sign_jwt(&user.id.to_hex(), &secret, exp)?;
    let url = format!("http://localhost:5173/auth/validate/{}/{}", 
        &user.id.to_hex(), &validation_token
    );

    let _ = acc_validation_service(
        &state.mailer_api_key, &state.mailer_service_url, &user.email, &url).await?
    ;

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
    Extension(refresh): Extension<String>, 
    Extension(user): Extension<UserModel>) -> HttpResponse {

    let _ = save_token("exp_tokens", &state.db, &token, user.id).await?;
    let _ = save_token("exp_tokens", &state.db, &refresh, user.id).await?;

    let session_cookie = new_cookie("token", "".to_string(), time::Duration::minutes(2));
    let refresh_cookie = new_cookie("refresh", "".to_string(), time::Duration::minutes(2));

    let _ = cookies.remove(session_cookie);
    let _ = cookies.remove(refresh_cookie);

    Ok(Response::LOGOUT_SUCCESS)
}

pub async fn validate_session() -> HttpResponse {
    Ok(Response::SUCCESS)
}

pub async fn protected() -> HttpResponse {
    Ok(Response::SUCCESS)
}
