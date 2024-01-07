
pub mod auth_controllers;
pub mod user_controllers;

use serde_json::json;
use axum::{response::IntoResponse, Json, extract::State};

use crate::config::app_state::AppState;

pub async fn api_health(State(state): State<AppState>) -> impl IntoResponse {
    let res = json!({
        "status": state.status,
        "url": format!("http://localhost:{}", state.api_port)
    });

    Json(res)
}
