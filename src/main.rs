mod config;
mod controllers;
mod models;
mod routes;
mod services;
mod utils;
mod middlewares;

use axum::routing::Router;

use config::app_state::AppState;
use config::database::db_connection;

use routes::api_router;
use routes::auth_routes::auth_router;
use routes::user_routes::user_router;

#[tokio::main]
async fn main() {
    
    let db_pool = db_connection().await;
    let app_state = AppState::new(db_pool.unwrap());

    let app = Router::new()
        .merge(api_router())
        .merge(auth_router())
        .merge(user_router())
        .layer(axum::extract::Extension(app_state.clone()));

    let server_addr = format!("0.0.0.0:{}", &app_state.api_port);
    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();

    println!("\n🦀 Server running on port {}", &app_state.api_port);
    println!("💻 Esperando peticiones cliente\n");
    axum::serve(listener, app).await.unwrap();
}
