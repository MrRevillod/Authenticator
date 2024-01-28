
use axum::routing::{Router, post};
use axum::middleware::from_fn_with_state as func;

use crate::{
    
    config::state::AppState, 
    
    controllers::authentication::{
        validate_account,
        login_controller, 
        logout_controller,
        register_controller, 
        validate_session,
        protected
    },

    middlewares::session::session_validation,
};

pub fn auth_router(state: AppState) -> Router {

    Router::new()

        .route("/login", post(login_controller))
        .route("/register", post(register_controller))
        
        .route("/logout", post(logout_controller)
            .route_layer(func(state.clone(), session_validation)))

        .route("/validate-account/:id/:token", post(validate_account))

        .route("/validate-session", post(validate_session)
            .route_layer(func(state.clone(), session_validation))
        )

        .route("/protected", post(protected)
            .route_layer(func(state.clone(), session_validation))
        )

        .with_state(state)
}

