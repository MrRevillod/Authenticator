
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;

use super::types::{ApiError, ApiSuccess};
use crate::models::responses_models::*;

impl IntoResponse for LoginSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "message": self.message,
            "token": self.token
        });
        
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl IntoResponse for RegisterSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "message": self.message,
            "url": self.url
        });
        
        (StatusCode::CREATED, Json(response)).into_response()
    }
}

impl IntoResponse for GetUserSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "user": self.user
        });
        
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl IntoResponse for GetUsersSuccess {
    fn into_response(self) -> Response {
        let response = json!({
            "users": self.users,
            "results": self.results
        });
        
        (StatusCode::OK, Json(response)).into_response()
    }
}

impl IntoResponse for ApiSuccess {
    fn into_response(self) -> Response {

        match self {
            ApiSuccess::Login(login_success_schema) => {
                login_success_schema.into_response()
            }

            ApiSuccess::Register(register_success_schema) => {
                register_success_schema.into_response()
            }

            ApiSuccess::Logout => {
                let response = json!({"message": "Logout success"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::AccountValidated => {
                let response = json!({"message": "Account validated"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::GetUser(get_user_success_schema) => {
                get_user_success_schema.into_response()
            }

            ApiSuccess::GetUsers(get_users_success_schema) => {
                get_users_success_schema.into_response()
            }

            ApiSuccess::UserUpdated => {
                let response = json!({"message": "User updated"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::UserDeleted => {
                let response = json!({"message": "User deleted"});
                (StatusCode::OK, Json(response)).into_response()
            }

            ApiSuccess::ProfileUpdated => {
                let response = json!({"message": "Profile updated"});
                (StatusCode::OK, Json(response)).into_response()
            }
        }
    }
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

            ApiError::InvalidToken => {
                let response = json!({"message": "Invalid token"});
                (StatusCode::UNAUTHORIZED, Json(response)).into_response()
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
        }
    }
}