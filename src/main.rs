mod controllers;
mod entities;
mod middlewares;
mod migrations;
pub mod routes;
mod utils;

use actix_web::{App, HttpServer, web::Data};
use sea_orm::DatabaseConnection;
use std::{env, io::Result};
use utils::db::establish_connection;
use utils::db::run_migrations;

struct AppState {
    db: DatabaseConnection,
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = establish_connection().await?;
    let app_state = Data::new(AppState {
        jwt_secret: env::var("JWT_SECRET").unwrap(),
        db: pool,
    });

    run_migrations(&app_state.db).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::configure_routes)
    })
    .bind(("0.0.0.0", 8005))?
    .run()
    .await
}
