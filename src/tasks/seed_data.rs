use std::collections::BTreeMap;

use loco_rs::{db, prelude::*};

use crate::app::App;

pub struct SeedData;
#[async_trait]
impl Task for SeedData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_data".to_string(),
            detail: "Seeding Data".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, vars: &BTreeMap<String, String>) -> Result<()> {
        let path = std::path::Path::new("src/fixtures");
        db::run_app_seed::<App>(&app_context.db, path).await
    }
}
