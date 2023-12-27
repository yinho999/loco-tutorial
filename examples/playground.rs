use eyre::Context;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use myapp::{app::App, models::_entities::articles};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // ctx is the app context that contains the database connection, etc
    let ctx = playground::<App>()
        .await
        .context("failed to create playground")?;
    // Add a new article
    let article = articles::ActiveModel {
        title: Set(Some("Hello World".to_string())),
        context: Set(Some("Hello World".to_string())),
        ..Default::default()
    };
    article.insert(&ctx.db).await?;
    // Find all articles
    let res = articles::Entity::find().all(&ctx.db).await?;
    println!("{:?}", res);
    println!("welcome to playground. edit me at `examples/playground.rs`");

    Ok(())
}
