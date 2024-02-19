
use axum::{routing::get, Router};

pub mod user;
pub mod authentication;

use crate::controllers::api_health;
use crate::config::state::AppState;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(api_health))
        .with_state(state)
}
