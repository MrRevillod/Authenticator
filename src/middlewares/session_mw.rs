
use axum::{
    extract::{Request, State}, 
    response::Response, 
    middleware::Next
};

use crate::utils::jwt_utils::*;
use crate::models::user_models::UserSchema;
use crate::utils::types::{ApiError, ApiState};

pub async fn session_validation(State(state): ApiState, 
    mut req: Request, next: Next) -> Result<Response, ApiError> {

    let authorization = req.headers().get("Authorization");
    let token = split_authorization(authorization);

    if let None = token {
        return Err(ApiError::Unauthorized)
    }

    let uuid = decode_jwt(&token.unwrap(), &state.jwt_secret).await?;

    let user = sqlx::query_as!(UserSchema,
        r#"SELECT * FROM User WHERE uuid = ?"#, uuid
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::Unauthorized)?;

    req.extensions_mut().insert(user);
    
    Ok(next.run(req).await)
}