
use axum::Json;
use bcrypt::hash;
use axum::extract::{State, Path};

use crate::responses::ApiResponse;
use crate::config::state::ApiState;
use crate::responses::{success::*, error::*};

use crate::models::validations::Validation;
use crate::models::user::{PublicUserData, UserSchema, UpdateUserSchema};

pub async fn get_users_controller(State(state): ApiState) -> 
    ApiResponse<ApiSuccess, ApiError> {
    
    let users = sqlx::query_as!(UserSchema, r#"SELECT * FROM User"#)
        .fetch_all(&state.db)
        .await
        .map_err(|_| return ApiError::UnexpectedError)?
    ;

    let json_response = GetUsersSuccess {
        results: users.len(),
        users,
    };

    Ok(ApiSuccess::GetUsers(json_response))
}

pub async fn get_user_controller(State(state): ApiState, 
    Path(uuid): Path<String>) -> ApiResponse<ApiSuccess, ApiError> {

    let user = sqlx::query_as!(UserSchema, 
        r#"SELECT * FROM User WHERE uuid = ?"#, uuid.to_string()
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    if user.len() == 0 {
        return Err(ApiError::UserNotFound);
    }

    let json_response = GetUserSuccess {
        user: user[0].clone()
    };

    Ok(ApiSuccess::GetUser(json_response))
}

pub async fn update_user_controller(State(state): ApiState, Path(uuid): 
    Path<String>, Json(body): Json<UpdateUserSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let _ = body.validate()?;

    let update = sqlx::query!(
        r#"UPDATE User SET username = ?, email = ?, password = ?, role = ?, 
           validated = ? WHERE uuid = ?"#,
        body.username,
        body.email,
        body.password,
        body.role,
        body.validated,
        uuid.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;
    
    if update.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(ApiSuccess::UserUpdated)
}

pub async fn update_profile_controller(State(state): ApiState, Path(uuid): 
    Path<String>, Json(body): Json<PublicUserData>) -> ApiResponse<ApiSuccess, ApiError> {

    let _ = body.validate()?;

    let password = match hash(&body.password, 6) {
        Ok(password) => password,
        Err(_) => return Err(ApiError::UnexpectedError),
    };

    let update = sqlx::query!(
        r#"UPDATE User SET username = ?, email = ?, password = ? WHERE uuid = ?"#,
        body.username,
        body.email,
        password,
        uuid.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|_| ApiError::UnexpectedError)?;

    if update.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(ApiSuccess::ProfileUpdated)
}

pub async fn delete_user_controller(State(state): ApiState, Path(uuid): 
    Path<String>) -> ApiResponse<ApiSuccess, ApiError> {

    let delete = sqlx::query!(
        r#"DELETE FROM User WHERE uuid = ?"#,
        uuid.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    if delete.rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    Ok(ApiSuccess::UserDeleted)
}
