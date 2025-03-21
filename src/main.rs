mod db;
mod users;

use actix_web::{
    web::{scope, Data}, App,
    HttpServer,
};
use sea_orm::DatabaseConnection;
use users::routes::user_routes;
use db::utils::establish_connection;


struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection().await?;
    let app_state = Data::new(AppState { db: pool });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(scope("/user").configure(user_routes))
    })
    .bind(("0.0.0.0", 8005))?
    .run()
    .await
}
