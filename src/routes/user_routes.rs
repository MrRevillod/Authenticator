use axum::Router;
use axum::routing::{delete, get, patch, put};

use crate::controllers::user_controllers::*;

pub fn user_router() -> Router {
    Router::new()
        .route("/users/", get(get_users_controller))
        .route("/users/:uuid", get(get_user_controller))
        .route("/users/:uuid", put(update_user_controller))
        .route("/users/:uuid", patch(update_profile_controller))
        .route("/users/:uuid", delete(delete_user_controller))
}
