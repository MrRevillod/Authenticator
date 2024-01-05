
use bcrypt::verify;
use axum::extract::Path;
use axum::http::Request;
use axum::{Extension, Json};

use crate::utils::jwt_utils::*;
use crate::services::auth_services::*;
use crate::models::auth_models::*;
use crate::models::responses_models::*;
use crate::models::user_models::UserSchema;
use crate::utils::types::{ApiState, ApiResponse, ApiError, ApiSuccess};

pub async fn login_controller(Extension(state): ApiState,
    Json(body): Json<LoginSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let user = sqlx::query_as!(
        UserSchema,
        r#"SELECT * FROM User WHERE email = ?"#,
        &body.email
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::UserNotFound)?;

    if let Err(_) = verify(&body.password, &user.password) {
        return Err(ApiError::InvalidCredentials);
    }

    if user.validated.as_bool() == false {
        return Err(ApiError::AccountNotValidated);
    }

    let token = sign_jwt(&user.uuid, &state.jwt_secret).await?;

    let json_response = LoginSuccessSchema {
        message: "Login success".to_string(),
        token
    };

    Ok(ApiSuccess::Login(json_response))
}


pub async fn register_controller(Extension(state): ApiState, Json(body): 
    Json<RegisterSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let user_exists = check_user_exists(&state.db, &body.username, &body.email).await?;

    if user_exists {
        return Err(ApiError::UserAlreadyExists);
    }
    
    let user_id = save_user(&state.db, &body).await?;
    let validation_url = create_validation_url(&state.db, &user_id, &state.jwt_secret).await?;

    let json_response = RegisterSuccessSchema {
        message: "User created".to_string(),
        url: validation_url
    };

    Ok(ApiSuccess::Register(json_response))
}

pub async fn logout_controller(Extension(state): ApiState,
    req: Request<axum::body::Body>) -> ApiResponse<ApiSuccess, ApiError> {

    let req = req.headers();
    let req_token = req.get("Authorization");

    if let None = req_token {
        return Err(ApiError::Unauthorized);
    }

    let token = req_token
        .unwrap().to_str()
        .unwrap().split(" ")
        .collect::<Vec<&str>>()[1];

    let uuid = decode_jwt(&token.to_string(), &state.jwt_secret).await?;
    let _ = save_expired_token(&state.db, &uuid, &token.to_string()).await?;
    
    Ok(ApiSuccess::Logout)
}

pub async fn validate_account_controller(Extension(state): ApiState, Path(
    (uuid, token)): Path<(String, String)>) -> ApiResponse<ApiSuccess, ApiError>{

    let user = sqlx::query_as!(UserSchema,
        r#"SELECT * FROM User WHERE uuid = ?"#,
        uuid.to_string()
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| ApiError::UnexpectedError)?;

    let secret = format!("{}{:?}", &state.jwt_secret, &user[0].validated.as_bool());
    let _ = decode_jwt(&token.to_string(), &secret).await?;

    let _ = sqlx::query!(
        r#"UPDATE User SET validated = true WHERE uuid = ?"#,
        uuid.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|_| ApiError::UnexpectedError)?;

    Ok(ApiSuccess::AccountValidated)
}
