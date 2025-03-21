// src/db.rs
use std::env;
use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> std::io::Result<DatabaseConnection> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "❌ DATABASE_URL not set"))?;
    match Database::connect(&database_url).await {
        Ok(conn) => Ok(conn),
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "❌ Database connection failed"))
        }
    }
}