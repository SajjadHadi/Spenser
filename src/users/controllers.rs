use crate::AppState;
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SignupRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[post("/signup")]
pub async fn signup(_: Data<AppState>, data: Json<SignupRequest>) -> impl Responder {
    HttpResponse::Ok().json(data)
}

#[post("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login API!")
}
