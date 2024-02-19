
pub mod user;
pub mod authentication;

use serde_json::json;
use axum::{response::IntoResponse, Json, extract::State, http::StatusCode};

use crate::config::state::AppState;

pub async fn api_health(State(state): State<AppState>) -> impl IntoResponse {
    let res = json!({
        "status": state.status,
        "url": format!("http://localhost:{}", state.api_port)
    });

    (StatusCode::OK, Json(res))
}
