use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Comments {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //
        // add column
        //
        /*
        manager
            .alter_table(
                Table::alter()
                    .table(Movies::Table)
                    .add_column_if_not_exists(integer(Movies::Rating).borrow_mut())
                    .to_owned(),
            )
            .await
        */

        manager
            .alter_table(
                Table::alter()
                    .table(Comments::Table)
                    .add_column_if_not_exists(integer(Comments::UserId).borrow_mut())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-comments-users")
                    .from(Comments::Table, Comments::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .clone(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //
        // delete column
        //
        manager
            .alter_table(
                Table::alter()
                    .table(Comments::Table)
                    .drop_column(Comments::UserId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
