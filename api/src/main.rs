
mod config;
mod controllers;
mod middlewares;
mod responses;
mod routes;
mod services;
mod models;

use config::state::*;
use config::database::db_connection;

use routes::{
    user::user_router,
    authentication::auth_router,
    account::account_router,
};

use axum::{
    routing::Router,
    http::{Method, HeaderValue},
    http::header::{ACCEPT, AUTHORIZATION, ORIGIN, CONTENT_TYPE},
};

use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_serve_static::ServeDir;
use include_dir::{Dir, include_dir};
use tower_cookies::CookieManagerLayer;

static STATIC_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/public");

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
            CLIENT_ADDR.parse::<HeaderValue>().unwrap()
        )
    ;
        
    let cookies = CookieManagerLayer::new();
    let static_service = ServeDir::new(&STATIC_DIR);

    let app = Router::new()
        .nest("/auth", auth_router(state.clone()))
        .nest("/users", user_router(state.clone()))
        .nest("/account", account_router(state.clone()))
        .nest_service("/", static_service)
        .layer(cors)
        .layer(cookies)
    ;

    let listener = TcpListener::bind(SERVER_ADDR.to_string()).await.unwrap();

    println!("\n🦀 Server running on {}", SERVER_ADDR.to_string());
    println!("💻 Esperando peticiones cliente\n");
    
    axum::serve(listener, app).await.unwrap();

}
