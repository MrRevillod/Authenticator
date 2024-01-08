
use axum::Json;
use bcrypt::hash;
use axum::extract::{State, Path};

use crate::models::responses_models::{GetUserSuccess, GetUsersSuccess};
use crate::utils::types::{ApiState, ApiResponse, ApiError, ApiSuccess};
use crate::utils::validator::{update_user_validation, update_profile_validation};
use crate::models::user_models::{UserSchema, UpdateUserSchema, UpdateProfileSchema};

pub async fn get_users_controller(State(state): ApiState) -> 
    ApiResponse<ApiSuccess, ApiError> {
    
    let users = sqlx::query_as!(UserSchema, r#"SELECT * FROM User"#)
        .fetch_all(&state.db)
        .await;

    match users {
        Ok(users) => {

            let json_response = GetUsersSuccess {
                results: users.len(),
                users,
            };

            Ok(ApiSuccess::GetUsers(json_response))
        }

        Err(_) => return Err(ApiError::UnexpectedError),
    }
}

pub async fn get_user_controller(State(state): ApiState, 
    Path(uuid): Path<String>) -> ApiResponse<ApiSuccess, ApiError> {

    let user = sqlx::query_as!(
        UserSchema,
        r#"SELECT * FROM User WHERE uuid = ?"#,
        uuid.to_string()
    )
    .fetch_all(&state.db)
    .await;

    match user {
        Ok(user) => {
            
            if user.len() == 0 {
                return Err(ApiError::UserNotFound)
            }

            let json_response = GetUserSuccess {
                user: user[0].clone(),
            };

            return Ok(ApiSuccess::GetUser(json_response))
        }

        Err(_) => return Err(ApiError::UserNotFound),
    }
}

pub async fn update_user_controller(State(state): ApiState, Path(uuid): 
    Path<String>, Json(body): Json<UpdateUserSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let _ = update_user_validation(&body).await?;

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
    .map_err(|_| ApiError::UnexpectedError);

    if let Err(_) = update {
        return Err(ApiError::UnexpectedError)
    }
    
    if update.unwrap().rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    return Ok(ApiSuccess::UserUpdated)
}

pub async fn update_profile_controller(State(state): ApiState, Path(uuid): 
    Path<String>, Json(body): Json<UpdateProfileSchema>) -> ApiResponse<ApiSuccess, ApiError> {

    let _ = update_profile_validation(&body).await?;

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
    .map_err(|_| ApiError::UnexpectedError);

    if let Err(_) = update {
        return Err(ApiError::UnexpectedError)
    }

    if update.unwrap().rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    return Ok(ApiSuccess::ProfileUpdated)
}

pub async fn delete_user_controller(State(state): ApiState, Path(uuid): 
    Path<String>) -> ApiResponse<ApiSuccess, ApiError> {

    let delete = sqlx::query!(
        r#"DELETE FROM User WHERE uuid = ?"#,
        uuid.to_string()
    )
    .execute(&state.db)
    .await
    .map_err(|_| ApiError::UnexpectedError);

    if let Err(_) = delete {
        return Err(ApiError::UnexpectedError)
    }

    if delete.unwrap().rows_affected() == 0 {
        return Err(ApiError::UserNotFound);
    }

    return Ok(ApiSuccess::UserDeleted)
}
