

use axum::{
    extract::{Request, State}, 
    response::Response, 
    middleware::Next
};

use crate::utils::jwt_utils::*;
use crate::utils::types::{ApiError, ApiState};

pub async fn session_validation(State(state): ApiState, 
    req: Request, next: Next) -> Result<Response, ApiError> {

    let authorization = req.headers().get("Authorization");
    let token = split_authorization(authorization);

    if let None = token {
        return Err(ApiError::Unauthorized)
    }

    let _ = decode_jwt(&token.unwrap(), &state.jwt_secret).await?;

    println!("session_validation middleware");

    Ok(next.run(req).await)
}