use crate::migrations::m20220101_000001_create_users_table::User;
use sea_orm_migration::{prelude::*, schema::*};
use crate::migrations::m20250323_095154_create_categories_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(pk_auto(Transaction::Id))
                    .col(integer(Transaction::UserId).not_null())
                    .col(integer(Transaction::CategoryId).not_null())
                    .col(string(Transaction::Type).not_null())
                    .col(big_integer(Transaction::Amount).not_null())
                    .col(string(Transaction::Memo).not_null())
                    .col(string_null(Transaction::Description))
                    .col(
                        timestamp(Transaction::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Transaction::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_user_id")
                            .from(Transaction::Table, Transaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_category_id")
                            .from(Transaction::Table, Transaction::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transaction {
    Table,
    Id,
    UserId,
    CategoryId,
    Type,
    Amount,
    Memo,
    Description,
    CreatedAt,
    UpdatedAt,
}
