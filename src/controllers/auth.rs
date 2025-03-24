use actix_web::{HttpResponse, Responder, post, web};
use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::PasswordHash,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::responses::unauthorized;
use crate::{
    AppState,
    entities::user::{ActiveModel as UserActiveModel, Entity as User},
};

#[derive(Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: i64,
}

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[post("/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> Result<impl Responder, actix_web::Error> {
    // Check if email exists
    let db = &state.db;
    if User::find()
        .filter(crate::entities::user::Column::Email.eq(&data.email))
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .is_some()
    {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "Email already exists."
        })));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Hash error: {}", e)))?
        .to_string();

    // Create user
    let new_user = UserActiveModel {
        email: Set(data.email.clone()),
        password: Set(hashed_password),
        first_name: Set(data.firstname.clone()),
        last_name: Set(data.lastname.clone()),
        balance: Set(0),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
        ..Default::default() // id auto-increments
    };
    new_user.insert(db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "message": "Account created successfully."
    })))
}

#[post("/sign-in")]
pub async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInRequest>,
) -> Result<impl Responder, actix_web::Error> {
    // Find user by email
    let user = User::find()
        .filter(crate::entities::user::Column::Email.eq(&data.email))
        .one(&state.db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| unauthorized("Invalid email or password"))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Hash error: {}", e)))?;
    Argon2::default()
        .verify_password(data.password.as_bytes(), &parsed_hash)
        .map_err(|_| unauthorized("Invalid email or password"))?;

    // Generate JWT
    let claims = Claims {
        sub: user.id,
        role: "user".to_string(),
        exp: (Utc::now() + Duration::hours(4)).timestamp(), // 4-hour expiration
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .map_err(|_| actix_web::error::ErrorInternalServerError("JWT_SECRET not set"))?
                .as_bytes(),
        ),
    )
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("JWT error: {}", e)))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "token": token
    })))
}
