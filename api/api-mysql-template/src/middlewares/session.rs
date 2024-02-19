
use axum::{
    extract::{Request, State}, 
    response::Response, 
    middleware::Next
};

use crate::utils::jwt::*;
use crate::models::user::UserSchema;

use crate::config::state::ApiState;
use crate::responses::error::ApiError;

pub async fn session_validation(State(state): ApiState, 
    mut req: Request, next: Next) -> Result<Response, ApiError> {

    let authorization = req.headers().get("Authorization");
    let token = split_authorization(authorization)?;

    let expired = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM ExpiredToken WHERE token = ?"#,
        &token
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| return ApiError::UnexpectedError)?;

    if expired.count > 0 {
        return Err(ApiError::Unauthorized)
    }

    let uuid = decode_jwt(&token, &state.jwt_secret)?;

    let user = sqlx::query_as!(UserSchema,
        r#"SELECT * FROM User WHERE uuid = ?"#, uuid
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::Unauthorized)?;

    req.extensions_mut().insert(user);
    req.extensions_mut().insert(token);
    
    Ok(next.run(req).await)
}