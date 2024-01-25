
use mongodb::Database;
use axum::extract::State;

use super::env;

pub type ApiState = State<AppState>;

#[derive(Clone)]
pub struct AppState {

    pub db: Database,
    pub jwt_secret: String,
    pub server_addr: String,
    pub client_addr: String,
}

impl AppState {

    pub fn new(db: Database) -> AppState {

        AppState {

            db,
            jwt_secret: env("JWT_SECRET"),
            
            server_addr: format!("{}:{}", env("SERVER_IP"), env("SERVER_PORT")),
            client_addr: format!("{}:{}", env("CLIENT_IP"), env("CLIENT_PORT")),
        }
    }

}