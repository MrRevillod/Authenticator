
use axum::Router;
use axum::routing::{delete, get, patch, put};

use crate::middlewares::*;
use crate::middlewares::roles::*;
use crate::middlewares::session::*;

use crate::controllers::user::*;
use crate::config::state::AppState;

use axum::middleware::{from_fn_with_state, from_fn};

pub fn user_router(state: AppState) -> Router {
    Router::new()
        .route("/users/", get(get_users_controller)

            .route_layer(from_fn(|req, next|
                role_validation(req, next, vec!["ADMIN_ROLE"])
            ))

            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
        )

        .route("/users/:uuid", get(get_user_controller)

            .route_layer(from_fn(owner_validation))
            
            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
            .route_layer(from_fn_with_state(state.clone(), uuid_validation))
        )
        
        .route("/users/:uuid", put(update_user_controller)

            .route_layer(from_fn(|req, next|
                role_validation(req, next, vec!["ADMIN_ROLE"])
            ))

            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
            .route_layer(from_fn_with_state(state.clone(), uuid_validation))
        )
        
        .route("/users/:uuid", patch(update_profile_controller)

            .route_layer(from_fn(owner_validation))

            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
            .route_layer(from_fn_with_state(state.clone(), uuid_validation))
        )
        
        .route("/users/:uuid", delete(delete_user_controller)

            .route_layer(from_fn(owner_validation))

            .route_layer(from_fn_with_state(
                state.clone(), session_validation)
            )
            .route_layer(from_fn_with_state(state.clone(), uuid_validation))
        )

    .with_state(state)
}
