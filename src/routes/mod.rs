use axum::{routing::get, Router};

pub mod auth_routes;
pub mod user_routes;

use crate::controllers::api_health;

pub fn api_router() -> Router {
    Router::new().route("/", get(api_health))
}
