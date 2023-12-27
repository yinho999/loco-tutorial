#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20231224_162701_posts;
mod m20231224_192255_articles;
mod m20231224_204122_comments;
mod m20231224_210108_add_comments_user_id;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20231224_162701_posts::Migration),
            Box::new(m20231224_192255_articles::Migration),
            Box::new(m20231224_204122_comments::Migration),
            Box::new(m20231224_210108_add_comments_user_id::Migration),
        ]
    }
}