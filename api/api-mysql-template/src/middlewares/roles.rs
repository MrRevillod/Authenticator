
use axum::{
    middleware::Next,
    response::Response,
    extract::Request
};

use crate::models::user::UserSchema;
use crate::responses::error::ApiError;

pub async fn role_validation(req: Request, next: Next, 
    roles: Vec<&str>) -> Result<Response, ApiError> {

    let user = req.extensions()
        .get::<UserSchema>()
        .ok_or(ApiError::Unauthorized)?;

    let user_role: &str = &user.role;

    if !roles.contains(&user_role) {
        return Err(ApiError::Unauthorized)
    }

    Ok(next.run(req).await)
}

pub async fn owner_validation(req: Request, next: Next) -> Result<Response, ApiError> {

    let user = req.extensions()
        .get::<UserSchema>()
        .ok_or(ApiError::Unauthorized)?;

    let user_uuid = &user.uuid;
    let path_uuid = req.uri().path().trim_start_matches("/users/");

    if path_uuid != user_uuid && user.role != "ADMIN_ROLE" {
        return Err(ApiError::Unauthorized)
    }

    Ok(next.run(req).await)
}