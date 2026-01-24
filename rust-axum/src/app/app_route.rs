use crate::app::app_config::AppConfig;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppRoute {
    pub config: AppConfig,
    pub db: Pool<Manager<PgConnection>>,
}

impl AppRoute {
    pub fn new(config: AppConfig, db: Pool<Manager<PgConnection>>) -> Self {
        Self { config, db }
    }
}
