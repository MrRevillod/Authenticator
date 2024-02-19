use axum::Router;
use axum::routing::{get, post};
use axum::middleware::from_fn_with_state;

use crate::config::state::AppState;
use crate::controllers::authentication::*;
use crate::middlewares::session::session_validation;

pub fn auth_router(state: AppState) -> Router {
    Router::new()
        
        .route("/auth/login", post(login_controller))
        .route("/auth/register", post(register_controller))
        .route("/auth/validate/:uuid/:token", get(validate_account_controller))
        
        .route("/auth/logout", post(logout_controller)
            .route_layer(from_fn_with_state(state.clone(), session_validation))
        )
        .route("/auth/refresh", post(refresh_token)
            .route_layer(from_fn_with_state(state.clone(), session_validation))
        )
        .with_state(state)
}
