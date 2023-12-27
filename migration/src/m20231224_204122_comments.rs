use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Comments::Table)
                    .col(pk_auto(Comments::Id).borrow_mut())
                    .col(text(Comments::Context).borrow_mut())
                    .col(integer(Comments::ArticleId).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comments-articles")
                            .from(Comments::Table, Comments::ArticleId)
                            .to(Articles::Table, Articles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    Context,
    ArticleId,
    
}


#[derive(DeriveIden)]
enum Articles {
    Table,
    Id,
}
