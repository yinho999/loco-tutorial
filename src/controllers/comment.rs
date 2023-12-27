#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::{
    comments::{ActiveModel, Model},
    users,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub context: Option<String>,
    pub article_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel, current_user: &users::Model) {
        item.context = Set(self.context.clone());
        item.article_id = Set(self.article_id);
        item.user_id = Set(current_user.id);
    }
}

pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    // we only want to make sure it exists
    let current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item, &current_user);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub fn routes() -> Routes {
    Routes::new().prefix("comments").add("/", post(add))
}
