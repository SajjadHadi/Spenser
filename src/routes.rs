use crate::controllers::auth::{sign_in, sign_up};
use actix_web::{web, web::ServiceConfig};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/auth").service(sign_in).service(sign_up));
}
