use axum::Router;
use axum::routing::{get, post};

use crate::config::app_state::AppState;
use crate::controllers::auth_controllers::*;

pub fn auth_router(state: AppState) -> Router {
    Router::new()
        .route("/auth/login", post(login_controller))
        .route("/auth/register", post(register_controller))
        .route("/auth/logout", post(logout_controller))
        .route("/auth/validate/:uuid/:token", get(validate_account_controller))
        .with_state(state)
}
