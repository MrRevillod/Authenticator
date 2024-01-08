

use axum::extract::State;

use crate::models::responses_models::*;
use crate::config::app_state::AppState;

// Response for controllers/handlers
pub type ApiResponse<T, U> = core::result::Result<T, U>;

// Result for services
pub type ApiResult<T> = ApiResponse<T, ApiError>;

// State for controllers/handlers
pub type ApiState = State<AppState>;

// Success response for controllers/handlers

#[derive(Debug)]
pub enum ApiSuccess {
    Register(RegisterSuccess),
    Login(LoginSuccess),
    Logout,
    AccountValidated,
    GetUser(GetUserSuccess),
    GetUsers(GetUsersSuccess),
    UserUpdated,
    UserDeleted,
    ProfileUpdated,
}

// Error response for controllers/handlers

#[derive(Debug)]
pub enum ApiError {
    UnexpectedError,
    Unauthorized,
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,
    AccountNotValidated,
    BadRequest
}

