use std::sync::Arc;

use axum::extract::FromRef;

// mod cache;
// pub mod db_pool;

// use cache::Cache;
// use db_pool::DatabasePool;

use crate::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<Config>,
    // /// Database connection pool
    // pub db: DatabasePool,
    // /// Concurrent cache
    // pub cache: Cache,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        Ok(AppState {
            config: Arc::new(config),
            // db: DatabasePool::new()?,
            // cache: Cache::new(),
        })
    }
}
