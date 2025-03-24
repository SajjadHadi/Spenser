use actix_web::{HttpRequest, HttpResponse, delete, get, post, put, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    AppState,
    entities::category::Entity as Category,
    entities::transaction::{
        ActiveModel as TransactionActiveModel, Entity as Transaction, Model as TransactionModel,
    },
    entities::user::Entity as User,
    utils::auth::get_user_by_id,
};

#[derive(Deserialize, Debug)]
pub struct CreateTransactionRequest {
    pub category_id: i32,
    pub r#type: String,
    pub amount: i64,
    pub memo: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTransactionRequest {
    pub memo: String,
    pub description: Option<String>,
}

#[get("")]
pub async fn get_all_transactions(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);

    let transactions = Transaction::find()
        .filter(crate::entities::transaction::Column::UserId.eq(user_id))
        .all(&state.db)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(transactions))
}

#[post("")]
pub async fn create_transaction(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateTransactionRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let db = &state.db;

    let (user, category) = fetch_user_and_category(db, user_id, data.category_id).await?;

    if data.r#type == "DEBIT" && (user.balance < data.amount || category.balance < data.amount) {
        return Err(actix_web::error::ErrorBadRequest(
            json!({"status": "error", "message": "Insufficient balance"}),
        ));
    }

    let transaction = TransactionActiveModel {
        user_id: Set(user_id),
        category_id: Set(data.category_id),
        r#type: Set(data.r#type.clone()),
        amount: Set(data.amount),
        memo: Set(data.memo.clone()),
        description: Set(data.description.clone()),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?;

    update_balances(
        db,
        user_id,
        data.category_id,
        &data.r#type,
        data.amount,
        false,
    )
    .await?;

    Ok(HttpResponse::Created().json(transaction))
}

#[get("/{id}")]
pub async fn show_transaction(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let transaction = find_transaction(&state.db, user_id, *id).await?;
    Ok(HttpResponse::Ok().json(transaction))
}

#[put("/{id}")]
pub async fn update_transaction(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
    data: web::Json<UpdateTransactionRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let transaction = find_transaction(&state.db, user_id, *id).await?;

    let mut updated_transaction: TransactionActiveModel = transaction.into();
    updated_transaction.memo = Set(data.memo.clone());
    updated_transaction.description = Set(data.description.clone());
    updated_transaction.updated_at = Set(Utc::now().into());

    let updated = updated_transaction.update(&state.db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(updated))
}

#[delete("/{id}")]
pub async fn delete_transaction(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = get_user_by_id(&req);
    let db = &state.db;

    let transaction = find_transaction(db, user_id, *id).await?;
    let (user, category) = fetch_user_and_category(db, user_id, transaction.category_id).await?;

    if is_credit(&transaction.r#type)
        && (transaction.amount > user.balance || transaction.amount > category.balance)
    {
        return Err(actix_web::error::ErrorBadRequest(
            json!({"status": "error", "message": "Insufficient balance"}),
        ));
    }

    Transaction::delete_by_id(*id).exec(db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    update_balances(
        db,
        user_id,
        transaction.category_id,
        &transaction.r#type,
        transaction.amount,
        true,
    )
    .await?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}

// Helpers
async fn fetch_user_and_category(
    db: &DatabaseConnection,
    user_id: i32,
    category_id: i32,
) -> Result<
    (
        crate::entities::user::Model,
        crate::entities::category::Model,
    ),
    actix_web::Error,
> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))?;

    let category = Category::find_by_id(category_id)
        .filter(crate::entities::category::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(
                json!({"status": "error", "message": "Category not found or unauthorized"}),
            )
        })?;

    Ok((user, category))
}

async fn find_transaction(
    db: &DatabaseConnection,
    user_id: i32,
    transaction_id: i32,
) -> Result<TransactionModel, actix_web::Error> {
    Transaction::find_by_id(transaction_id)
        .filter(crate::entities::transaction::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(
                json!({"status": "error", "message": "Transaction not found or unauthorized"}),
            )
        })
}

async fn update_balances(
    db: &DatabaseConnection,
    user_id: i32,
    category_id: i32,
    r#type: &str,
    amount: i64,
    is_delete: bool,
) -> Result<(), actix_web::Error> {
    let (user_delta, category_delta) = match (r#type, is_delete) {
        ("DEBIT", false) => (-amount, -amount),
        ("CREDIT", false) => (amount, amount),
        ("DEBIT", true) => (amount, amount),
        ("CREDIT", true) => (-amount, -amount),
        _ => {
            return Err(actix_web::error::ErrorBadRequest(
                "Invalid transaction type",
            ));
        }
    };

    let mut user = User::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?
        .into_active_model();
    user.balance = Set(user.balance.unwrap() + user_delta);
    user.update(db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    let mut category = Category::find_by_id(category_id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Category not found"))?
        .into_active_model();
    category.balance = Set(category.balance.unwrap() + category_delta);
    category.update(db).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    Ok(())
}

fn is_credit(r#type: &str) -> bool {
    r#type == "CREDIT"
}
