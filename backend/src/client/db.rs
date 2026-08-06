use crate::{config::Config, error::AppError};

use std::error::Error;

use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct Database {
    inner: Pool,
}

impl Database {
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config
            .dbname(&config.db_name)
            .user(&config.db_user)
            .password(&config.db_pass)
            .hostaddr(config.db_addr);
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
        let pool = Pool::builder(mgr).max_size(config.db_pool_size).build()?;
        Ok(Self { inner: pool })
    }

    pub async fn get(&self) -> Result<Object, AppError> {
        self.inner.get().await.map_err(|e| e.into())
    }
}
