use crate::migrations::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::env;
use std::io;

pub async fn run_migrations(db: &DatabaseConnection) -> io::Result<()> {
    Migrator::up(db, None)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("❌ Migration failed: {}", e)))?;
    println!("✅ Migrations applied successfully");
    Ok(())
}

pub async fn establish_connection() -> std::io::Result<DatabaseConnection> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "❌ DATABASE_URL not set"))?;

    let mut opt = ConnectOptions::new(database_url);
    opt.sqlx_logging(false);

    match Database::connect(opt).await {
        Ok(conn) => {
            println!("✅ Database connected");
            Ok(conn)
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "❌ Database connection failed",
            ))
        }
    }
}
