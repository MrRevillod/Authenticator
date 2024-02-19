
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug)]
pub enum ApiError {
    UnexpectedError,
    Unauthorized,
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,
    AccountNotValidated,
    BadRequest,
    DataValidationError(HashMap<&'static str, &'static str>),
    RefreshTokenConflict,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {

        println!("❌ ApiError: {:?}", self);

        match self {
            ApiError::UnexpectedError => {
                let response = json!({"message": "Internal server error"});
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            }

            ApiError::Unauthorized => {
                let response = json!({"message": "Unauthorized"});
                (StatusCode::UNAUTHORIZED, Json(response)).into_response()
            }

            ApiError::UserNotFound => {
                let response = json!({"message": "User not found"});
                (StatusCode::NOT_FOUND, Json(response)).into_response()
            }

            ApiError::UserAlreadyExists => {
                let response = json!({"message": "User already exists"});
                (StatusCode::CONFLICT, Json(response)).into_response()
            }

            ApiError::InvalidCredentials => {
                let response = json!({"message": "Invalid credentials"});
                (StatusCode::UNAUTHORIZED, Json(response)).into_response()
            }

            ApiError::AccountNotValidated => {
                let response = json!({"message": "Account not validated"});
                (StatusCode::UNAUTHORIZED, Json(response)).into_response()
            }

            ApiError::BadRequest => {
                let response = json!({"message": "Bad request"});
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }

            ApiError::DataValidationError(errors) => {
                let response = json!({"message": "Validation error", "errors": errors});
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }

            ApiError::RefreshTokenConflict => {
                let response = json!({"message": "El token enviado es un refresh token"});
                (StatusCode::CONFLICT, Json(response)).into_response()
            }
        }
    }
}