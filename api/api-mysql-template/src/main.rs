mod config;
mod controllers;
mod models;
mod routes;
mod services;
mod utils;
mod middlewares;
mod responses;

use axum::http::Method;
use axum::routing::Router;
use tower_http::cors::{CorsLayer, Any};
use axum::http::header::{ORIGIN, AUTHORIZATION, ACCEPT};

use config::state::AppState;
use config::database::db_connection;

use routes::api_router;
use routes::user::user_router;
use routes::authentication::auth_router;

#[tokio::main]
async fn main() {
    
    let db_pool = db_connection().await;
    let app_state = AppState::new(db_pool.unwrap());

    let http_headers = vec![ORIGIN, AUTHORIZATION, ACCEPT];
    let http_methods = vec![Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE];

    let cors = CorsLayer::new()
        .allow_methods(http_methods)
        .allow_headers(http_headers)
        .allow_origin(Any)
    ;

    let app = Router::new()
        .merge(api_router(app_state.clone()))
        .merge(auth_router(app_state.clone()))
        .merge(user_router(app_state.clone()))
        .layer(cors)
    ;

    let server_addr = format!("0.0.0.0:{}", &app_state.api_port);
    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();

    println!("\n🦀 Server running on port {}", &app_state.api_port);
    println!("💻 Esperando peticiones cliente\n");
    axum::serve(listener, app).await.unwrap();
}
