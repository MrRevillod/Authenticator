
use std::env;
use axum::extract::State;
use sqlx::mysql::MySqlPool;

pub type ApiState = State<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub api_port: String,
    pub api_key: String,
    pub jwt_secret: String,
    pub mysql_uri: String,
    pub status: String,
    pub api_domain: String,
}

impl AppState {
    pub fn new(db_pool: MySqlPool) -> AppState {
        AppState {
            db: db_pool,
            api_port: env::var("PORT").expect("El puerto debe estar configurado"),
            api_key: env::var("API_KEY").expect("La api key debe estar configurada"),
            jwt_secret: env::var("JWT_SECRET").expect("La clave de jwt debe estar configurada"),
            mysql_uri: env::var("DATABASE_URL").expect("La mysql_uri debe estar configurada"),
            status: "running".to_string(),
            api_domain: "http://localhost:4000".to_string(),
        }
    }
}
