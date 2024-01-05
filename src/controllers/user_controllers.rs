
use serde_json::json;

use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension, Json
};

use crate::models::user_models::UserSchema;
use crate::utils::types::{ApiState, ApiResponse, ApiError};

pub async fn get_users_controller(Extension(state): ApiState) -> 
    ApiResponse<impl IntoResponse, ApiError> {
    
    let users = sqlx::query_as!(UserSchema, r#"SELECT * FROM User"#)
        .fetch_all(&state.db)
        .await;

    match users {
        Ok(users) => {
            
            let response = json!({
                "message": "OK",
                "users": users,
                "results": users.len()
            });

            Ok((StatusCode::OK, Json(response)))
        }

        Err(_) => return Err(ApiError::UnexpectedError),
    }
}

pub async fn get_user_controller(Extension(state): ApiState, 
    Path(uuid): Path<String>) -> ApiResponse<impl IntoResponse, ApiError> {

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

            let single_user = &user[0];
            let response = json!({"message": "OK", "user": single_user});
            return Ok((StatusCode::OK, Json(response)))
        }

        Err(_) => return Err(ApiError::UserNotFound),
    }
}

pub async fn update_user_controller() -> impl IntoResponse {
    let res = json!({ "message": "user update success" });
    Json(res)
}

pub async fn update_profile_controller() -> impl IntoResponse {
    let res = json!({ "message": "profile update success" });
    Json(res)
}

pub async fn delete_user_controller() -> impl IntoResponse {
    let res = json!({ "message": "user deleted" });
    Json(res)
}
