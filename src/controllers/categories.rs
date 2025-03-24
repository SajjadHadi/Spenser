use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use serde_json::json;

use crate::utils::auth::get_user_by_id;
use crate::{
    AppState,
    entities::category::{
        ActiveModel as CategoryActiveModel, Entity as Category, Model as CategoryModel,
    },
    entities::transaction::Entity as Transaction,
};

#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[get("")]
pub async fn get_all_categories(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);

    let categories = Category::find()
        .filter(crate::entities::category::Column::UserId.eq(user_id))
        .all(&state.db)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(categories))
}

#[post("")]
pub async fn create_category(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateCategoryRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);

    let new_category = CategoryActiveModel {
        user_id: Set(user_id),
        name: Set(data.name.clone()),
        description: Set(data.description.clone()),
        balance: Set(0),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
        ..Default::default()
    };

    let category = new_category.insert(&state.db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(category))
}

#[get("/{id}")]
pub async fn show_category(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);

    let category = find_category(&state.db, user_id, *id).await?;
    Ok(HttpResponse::Ok().json(category))
}

#[put("/{id}")]
pub async fn update_category(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
    data: web::Json<UpdateCategoryRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let category = find_category(&state.db, user_id, *id).await?;

    let mut updated_category: CategoryActiveModel = category.into();
    updated_category.name = Set(data.name.clone());
    updated_category.description = Set(data.description.clone());
    updated_category.updated_at = Set(chrono::Utc::now().into());

    let updated = updated_category.update(&state.db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/{id}")]
pub async fn delete_category(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let category = find_category(&state.db, user_id, *id).await?;

    Category::delete_by_id(category.id)
        .exec(&state.db)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}

#[get("/{id}/transactions")]
pub async fn get_category_transactions(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let _category = find_category(&state.db, user_id, *id).await?;

    let transactions = Transaction::find()
        .filter(crate::entities::transaction::Column::CategoryId.eq(*id))
        .all(&state.db)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(transactions))
}

async fn find_category(
    db: &DatabaseConnection,
    user_id: i32,
    category_id: i32,
) -> Result<CategoryModel, actix_web::Error> {
    Category::find_by_id(category_id)
        .filter(crate::entities::category::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(
                json!({"status": "error", "message": "Not found or unauthorized"}),
            )
        })
}
