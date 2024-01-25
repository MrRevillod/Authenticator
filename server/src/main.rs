
mod config;
mod controllers;
mod middlewares;
mod responses;
mod routes;
mod services;
mod models;

use config::state::AppState;
use config::database::db_connection;

use routes::authentication::auth_router;

use axum::routing::Router;
use axum::http::{Method, HeaderValue};
use axum::http::header::{ACCEPT, AUTHORIZATION, ORIGIN, CONTENT_TYPE};

use tokio::net::TcpListener;

use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {

    let database = match db_connection().await {
        
        Ok(db) => db,
        Err(e) => {
            println!("🔥 Error al conectar con la base de datos: {}", e);
            return
        }
    };

    let state = AppState::new(database);
    let http_headers = vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE];

    let http_methods = vec![
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
    ];

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(http_methods)
        .allow_headers(http_headers)
        .allow_origin(
            state.client_addr.parse::<HeaderValue>().unwrap()
        )
    ;

    let cookies = CookieManagerLayer::new();

    let app = Router::new()
        .nest("/auth", auth_router(state.clone()))
        .layer(cors)
        .layer(cookies)
    ;

    let listener = TcpListener::bind(&state.server_addr).await.unwrap();

    println!("\n🦀 Server running on {}", &state.server_addr);
    println!("💻 Esperando peticiones cliente\n");
    
    axum::serve(listener, app).await.unwrap();

}
