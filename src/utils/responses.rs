use actix_web::Error;
use serde_json::json;

pub fn unauthorized(message: &str) -> Error {
    actix_web::error::ErrorUnauthorized(json!({
        "status": "error",
        "message": message
    }))
}