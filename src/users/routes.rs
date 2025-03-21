use crate::users::controllers::{login, signup};
use actix_web::web::ServiceConfig;

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg.service(signup).service(login); // directly use the route handlers
}
