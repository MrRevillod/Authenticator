
use std::sync::Arc;

use axum::Extension;

use crate::models::responses_models::*;
use crate::config::app_state::AppState;

// Response for controllers/handlers
pub type ApiResponse<T, U> = core::result::Result<T, U>;

// Result for services
pub type ApiResult<T> = ApiResponse<T, ApiError>;

// State for controllers/handlers
pub type ApiState = Extension<Arc<AppState>>;

// Success response for controllers/handlers

#[derive(Debug)]
pub enum ApiSuccess {
    Register(RegisterSuccessSchema),
    Login(LoginSuccessSchema),
    Logout,
    AccountValidated,
}

// Error response for controllers/handlers

#[derive(Debug)]
pub enum ApiError {
    UnexpectedError,
    Unauthorized,
    UserNotFound,
    UserAlreadyExists,
    InvalidToken,
    InvalidCredentials,
    AccountNotValidated,
}

