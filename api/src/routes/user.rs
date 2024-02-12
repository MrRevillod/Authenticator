
use axum::{
    
    middleware::{
        from_fn as mw,
        from_fn_with_state as mw_func,
    },
    
    routing::{get, delete, patch, Router},
};

use crate::{
    
    controllers::user::*, 
    config::state::AppState, 
    
    middlewares::{
        session::session_validation, 
        validation::{is_valid_id, owner_validation}
    } 
};

pub fn user_router(state: AppState) -> Router {

    Router::new()

        .route("/:id", get(get_user)
            .route_layer(mw_func(state.clone(), session_validation))
            .route_layer(mw_func(state.clone(), is_valid_id))
        )
        
        .route("/:id", delete(delete_account)
            .route_layer(mw(owner_validation))
            .route_layer(mw_func(state.clone(), session_validation))
            .route_layer(mw_func(state.clone(), is_valid_id))
        )

        .route("/:id", patch(update_profile)
            .route_layer(mw(owner_validation))
            .route_layer(mw_func(state.clone(), session_validation))
            .route_layer(mw_func(state.clone(), is_valid_id))
        )

        .route("/update-email/:id/:token", get(update_email)
            .route_layer(mw_func(state.clone(), is_valid_id)) 
        )

        .with_state(state)
}
