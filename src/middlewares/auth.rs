use actix_web::{
    Error, HttpMessage,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use actix_web::body::BoxBody;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::utils::responses::unauthorized;
use crate::{AppState, controllers::auth::Claims};

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {
    let token = extract_token(&req)?;
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| unauthorized("Missing app state"))?;
    let claims = validate_token(token, &state.jwt_secret)?;

    req.extensions_mut().insert(claims.sub);
    next.call(req).await
}

fn extract_token(req: &ServiceRequest) -> Result<&str, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| unauthorized("Authorization header is missing"))?
        .to_str()
        .map_err(|_| unauthorized("Authorization header is malformed"))?;

    auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| unauthorized("Authorization header must start with 'Bearer '"))
}

fn validate_token(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|_| unauthorized("Invalid token"))
}
