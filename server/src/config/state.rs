
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
    pub mailer_service_url: String,
    pub mailer_api_key: String,
}

impl AppState {

    pub fn new(db: Database) -> AppState {

        AppState {

            db,
            jwt_secret: env("JWT_SECRET"),
            
            server_addr: format!("{}:{}", env("SERVER_IP"), env("SERVER_PORT")),
            client_addr: format!("{}:{}", env("CLIENT_IP"), env("CLIENT_PORT")),
            mailer_service_url: env("MAILER_SERVICE_URL"),
            mailer_api_key: env("MAILER_API_KEY"),
        }
    }

}