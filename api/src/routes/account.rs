
use axum::{
    
    middleware::{
        from_fn as mw,
        from_fn_with_state as mw_func,
    },
    
    routing::{delete, get, patch, post, Router},
};

use crate::{
    
    config::state::AppState, 
    controllers::account::*, 
    middlewares::{
        session::session_validation, 
        validation::{
            is_valid_id, is_valid_id_and_token, owner_validation
        }
    } 
};

pub fn account_router(state: AppState) -> Router {

    Router::new()

        // validate the recent created account

        .route("/validate/:id/:token", post(validate_account)
            .route_layer(mw_func(state.clone(), is_valid_id_and_token))
        )

        // Update profile data
        
        .route("/:id", patch(update_account)
            .route_layer(mw(owner_validation))
            .route_layer(mw_func(state.clone(), session_validation))
            .route_layer(mw_func(state.clone(), is_valid_id))
        )

        // confirm account email update
        
        .route("/update-email/:id/:token", get(update_email)
            .route_layer(mw_func(state.clone(), is_valid_id_and_token)) 
        )

        // delete account
    
        .route("/:id", delete(delete_account)
            .route_layer(mw(owner_validation))
            .route_layer(mw_func(state.clone(), session_validation))
            .route_layer(mw_func(state.clone(), is_valid_id))
        )

    .with_state(state)
}