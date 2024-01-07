use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, patch, put};

use crate::middlewares::session_mw::*;

use crate::config::app_state::AppState;
use crate::controllers::user_controllers::*;

pub fn user_router(state: AppState) -> Router {
    Router::new()
        .route("/users/", get(get_users_controller)
            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
        )

        .route("/users/:uuid", get(get_user_controller)
            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
        )
        
        .route("/users/:uuid", put(update_user_controller)
            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
        )
        
        .route("/users/:uuid", patch(update_profile_controller)
            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
        )
        
        .route("/users/:uuid", delete(delete_user_controller)
            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
        )

    .with_state(state)
}
