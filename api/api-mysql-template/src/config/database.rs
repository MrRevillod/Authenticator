
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn db_connection() -> Result<MySqlPool, sqlx::Error> {
    
    dotenv().ok();

    let mysql_uri = std::env::var("DATABASE_URL")
        .expect("La mysql_uri debe estar configurada")
    ;

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&mysql_uri)
        .await
    ;

    match pool {
        
        Ok(pool) => {
            print!("\n🐬 Conexión exitosa con la base de datos");
            Ok(pool)
        }
        Err(e) => {
            println!("🐬 Error al conectar con la base de datos mysql");
            Err(e)
        }
    }
}
