mod db;
mod migrations;
mod users;

use crate::db::utils::run_migrations;
use actix_web::{
    App, HttpServer,
    web::{Data, scope},
};
use db::utils::establish_connection;
use sea_orm::DatabaseConnection;
use users::routes::user_routes;

struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection().await?;
    let app_state = Data::new(AppState { db: pool });

    run_migrations(&app_state.db).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(scope("/user").configure(user_routes))
    })
    .bind(("0.0.0.0", 8005))?
    .run()
    .await
}
