pub mod auth_controllers;
pub mod user_controllers;

use std::sync::Arc;
use serde_json::json;
use axum::{response::IntoResponse, Extension, Json};

use crate::config::app_state::AppState;

pub async fn api_health(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let res = json!({
        "status": "running",
        "url": format!("http://localhost:{}", state.api_port)
    });

    Json(res)
}
