use crate::middlewares::auth::verify_jwt;
use crate::{
    controllers::auth::{sign_in, sign_up},
    controllers::categories::{
        create_category, delete_category, get_all_categories, get_category_transactions,
        show_category, update_category,
    },
    controllers::transactions::{
        create_transaction, delete_transaction, get_all_transactions, show_transaction,
        update_transaction,
    },
};
use actix_web::middleware::from_fn;
use actix_web::web::{ServiceConfig, scope};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").service(sign_in).service(sign_up))
        .service(
            scope("/api")
                .service(
                    scope("/categories")
                        .wrap(from_fn(verify_jwt))
                        .service(get_all_categories)
                        .service(create_category)
                        .service(show_category)
                        .service(update_category)
                        .service(delete_category)
                        .service(get_category_transactions),
                )
                .service(
                    scope("/transactions")
                        .wrap(from_fn(verify_jwt))
                        .service(get_all_transactions)
                        .service(create_transaction)
                        .service(show_transaction)
                        .service(update_transaction)
                        .service(delete_transaction),
                ),
        );
}
