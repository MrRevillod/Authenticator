
use super::env;

use mongodb::{
    Client, Database,
    error::Error as MongoError,
    options::{ClientOptions, ResolverConfig},
};

pub async fn db_connection() -> Result<Database, MongoError> {

    let uri = env("DB_URI");

    let options =
        ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare()).await?
    ;

    let client = Client::with_options(options)?;
    let db = client.database("Workflow");

    println!("🚀 Conectado a la base de datos");

    Ok(db)
}