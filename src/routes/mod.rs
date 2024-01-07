use axum::{routing::get, Router, middleware::from_fn_with_state};

pub mod auth_routes;
pub mod user_routes;

use crate::{controllers::api_health, config::app_state::AppState, middlewares::__mw__};

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(api_health))
        .route_layer(from_fn_with_state(state.clone(), __mw__))
        .with_state(state)
}
