
use axum::{
    routing::{get, Router},
    middleware::from_fn_with_state as mw_func,
};

use crate::{
    
    controllers::user::*, 
    config::state::AppState, 
    
    middlewares::{
        session::session_validation, 
        validation::is_valid_id,
    } 
};

pub fn user_router(state: AppState) -> Router {

    Router::new()

        .route("/:id", get(get_user)
            .route_layer(mw_func(state.clone(), session_validation))
            .route_layer(mw_func(state.clone(), is_valid_id))
        )
        
    .with_state(state)
}
