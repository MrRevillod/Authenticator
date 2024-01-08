
pub mod role_mw;
pub mod session_mw;

use uuid::Uuid;

use axum::{
    middleware::Next, 
    response::Response,
    extract::{Request, Path},
};

use crate::utils::types::ApiError;

pub async fn uuid_validation(Path(uuid): Path<String>, 
    req: Request, next: Next) -> Result<Response, ApiError> {

    match Uuid::parse_str(&uuid) {
        Ok(_) => (),
        Err(_) => return Err(ApiError::BadRequest)
    }
    
    Ok(next.run(req).await)
}