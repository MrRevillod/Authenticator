
use mongodb::Database;
use axum::extract::State;

use lazy_static::lazy_static;

lazy_static! {

    pub static ref JWT_SECRET: String = env("JWT_SECRET").to_string();
    pub static ref SERVER_ADDR: String = format!("{}:{}", env("SERVER_IP"), env("SERVER_PORT")).to_string();
    pub static ref CLIENT_ADDR: String = format!("{}:{}", env("CLIENT_IP"), env("CLIENT_PORT")).to_string();
    pub static ref MAILER_SERVICE_URL: String = env("MAILER_SERVICE_URL").to_string();
    pub static ref MAILER_API_KEY: String = env("MAILER_API_KEY").to_string();
}

use super::env;
pub type ApiState = State<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

impl AppState {

    pub fn new(db: Database) -> AppState {
        AppState { db }
    }
}