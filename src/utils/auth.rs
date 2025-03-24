use actix_web::{HttpMessage, HttpRequest};

pub fn get_user_by_id(req: &HttpRequest) -> i32 {
    req.extensions().get::<i32>().copied().unwrap()
}
