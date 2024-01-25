
use axum::routing::{Router, post, get};
use axum::middleware::from_fn_with_state as func;

use crate::controllers::authentication::{refresh_token, validate_session};
use crate::{
    
    config::state::AppState, 
    
    controllers::authentication::{
        validate_account,
        login_controller, 
        logout_controller,
        register_controller, 
    },

    middlewares::session::session_validation,
};

pub fn auth_router(state: AppState) -> Router {

    Router::new()

        .route("/login", post(login_controller))
        .route("/register", post(register_controller))
        
        .route("/logout", post(logout_controller)
            .route_layer(func(state.clone(), session_validation)))

        .route("/validate-account/:id/:token", get(validate_account))

        .route("/refresh", get(refresh_token)
            .route_layer(func(state.clone(), session_validation)) 
        )

        .route("/validate-session", post(validate_session)
            .route_layer(func(state.clone(), session_validation))
        )

        .with_state(state)
}

