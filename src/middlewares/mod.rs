
pub mod session_mw;

use axum::{extract::{State, Request}, middleware::Next, response::Response};

use crate::{config::app_state::AppState, utils::types::ApiError};

pub async fn __mw__(State(state): State<AppState>, 
    req: Request, next: Next) -> Result<Response, ApiError> {

    println!("__mw__: {}", state.jwt_secret);
    Ok(next.run(req).await)
}