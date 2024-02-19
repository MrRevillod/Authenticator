
pub mod roles;
pub mod session;

use uuid::Uuid;

use axum::{
    middleware::Next, 
    response::Response,
    extract::{Request, Path, State},
};

use crate::config::state::ApiState;
use crate::responses::error::ApiError;

pub async fn uuid_validation(State(state): ApiState, Path(uuid): Path<String>, 
    req: Request, next: Next) -> Result<Response, ApiError> {

    match Uuid::parse_str(&uuid) {
        Ok(_) => (),
        Err(_) => return Err(ApiError::BadRequest)
    }

    match sqlx::query!(
        r#"SELECT uuid FROM User WHERE uuid = ?"#, uuid
    )
    .fetch_one(&state.db)
    .await {
        Ok(_) => (),
        Err(_) => return Err(ApiError::UserNotFound)
    }
    
    Ok(next.run(req).await)
}