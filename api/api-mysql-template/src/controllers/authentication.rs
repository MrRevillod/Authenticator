
use uuid::Uuid;
use axum::{Json, Extension};
use axum::extract::{Path, State};

use crate::utils::jwt::*;
use crate::config::state::ApiState;
use crate::services::authentication::*;

use crate::models::validations::Validation;
use crate::models::authentication::LoginSchema; 
use crate::models::user::{PublicUserData, SqlxBool, UserSchema};
use crate::responses::{ApiResponse, success::*, error::ApiError};

pub async fn login_controller(State(state): ApiState,
    Json(body): Json<LoginSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let user = sqlx::query!(
        r#"SELECT uuid, email, password, validated FROM User WHERE email = ?"#,
        &body.email
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| return ApiError::InvalidCredentials)?;

    let match_password = bcrypt::verify(&body.password, &user.password).unwrap();

    if !match_password {
        return Err(ApiError::InvalidCredentials);
    }

    let acc_validated = SqlxBool::from(user.validated).as_bool();

    if !acc_validated {
        return Err(ApiError::AccountNotValidated);
    }

    let token_exp = chrono::Utc::now() + chrono::Duration::hours(1);
    let token = sign_jwt(&user.uuid, &state.jwt_secret, token_exp)?;

    let json_response = LoginSuccess {
        message: "Login success",
        token
    };

    Ok(ApiSuccess::Login(json_response))
}

pub async fn register_controller(State(state): ApiState, Json(body): 
    Json<PublicUserData>) -> ApiResponse<ApiSuccess, ApiError> {

    let _ = body.validate()?;
    let exists = check_user_exists(&state.db, &body.username, &body.email).await?;

    if exists {
        return Err(ApiError::UserAlreadyExists);
    }
    
    let user_id = save_user(&state.db, &body).await?;
    let url = create_validation_url(&state.db, &user_id, &state.jwt_secret).await?;

    let json_response = RegisterSuccess {
        message: "User created",
        url
    };

    Ok(ApiSuccess::Register(json_response))
}

pub async fn logout_controller(State(state): ApiState, 
    Extension(token): Extension<String>) -> ApiResponse<ApiSuccess, ApiError> {

    let uuid = decode_jwt(&token, &state.jwt_secret)?;
    let _ = save_expired_token(&state.db, &uuid, &token).await?;
    
    Ok(ApiSuccess::Logout)
}

pub async fn validate_account_controller(State(state): ApiState, Path(
    (uuid, token)): Path<(String, String)>) -> ApiResponse<ApiSuccess, ApiError>{

    let user = sqlx::query!(
        r#"SELECT validated FROM User WHERE uuid = ?"#,
        uuid.to_string()
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    let validated = SqlxBool::from(user[0].validated).as_bool();

    let secret = format!("{}{:?}", &state.jwt_secret, &validated);
    let _ = decode_jwt(&token.to_string(), &secret)?;

    let _ = sqlx::query!(
        r#"UPDATE User SET validated = true WHERE uuid = ?"#,
        uuid.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    Ok(ApiSuccess::AccountValidated)
}

pub async fn refresh_token(State(state): ApiState, Extension(token): Extension<String>, 
    Extension(user): Extension<UserSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let refresh = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM RefreshToken WHERE token = ?"#,
        &token
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    if refresh.count > 0 {
        return Err(ApiError::RefreshTokenConflict);
    }

    let _ = save_expired_token(&state.db, &user.uuid, &token).await?;

    let exp = chrono::Utc::now() + chrono::Duration::days(1);
    let refresh_token = sign_jwt(&user.uuid, &state.jwt_secret, exp)?;

    let _ = sqlx::query!(
        r#"INSERT INTO RefreshToken (token_id, token, userId) VALUES (?, ?, ?)"#,
        Uuid::new_v4().to_string(),
        &refresh_token,
        &user.uuid
    )
    .execute(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    let json_response = LoginSuccess {
        message: "Refresh success",
        token: refresh_token
    };

    Ok(ApiSuccess::Refresh(json_response))
}

