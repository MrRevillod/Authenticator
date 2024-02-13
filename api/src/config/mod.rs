
pub mod state;
pub mod database;

use dotenv::dotenv;

pub fn env(key: &str) -> String {
    
    dotenv().ok();
    dotenv::var(key).expect(&format!("{} not found in .env file", key))
}