
use axum::routing::{patch, post, Router};
use axum::middleware::from_fn_with_state as func;

use crate::controllers::authentication::reset_password;
use crate::{
    
    config::state::AppState, 
    
    controllers::authentication::{
        login_controller, 
        logout_controller,
        register_controller, 
        validate_session,
        reset_password_validation,
        send_reset_password_email,
    },

    middlewares::session::session_validation,
};

pub fn auth_router(state: AppState) -> Router {

    Router::new()

        // login and get session tokens

        .route("/login", post(login_controller))

        // register in app

        .route("/register", post(register_controller))

        // close the current session
        
        .route("/logout", post(logout_controller)
            .route_layer(func(state.clone(), session_validation))
        )

        // return the status of current session

        .route("/validate", post(validate_session)
            .route_layer(func(state.clone(), session_validation))
        )

        // get the email from user and return one time link
        // for recover his password
        
        .route("/reset-password", post(send_reset_password_email))
        
        // validate the params from the url
        
        .route("/reset-password/:id/:token", post(reset_password_validation))

        // get the new password and patch it

        .route("/reset-password/:id/:token", patch(reset_password))

    .with_state(state)
}

