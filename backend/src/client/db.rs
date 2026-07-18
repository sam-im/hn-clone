use crate::config::Config;

use std::error::Error;

use deadpool_postgres::{Manager, ManagerConfig, Object, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use tracing::{debug, error};

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

    pub async fn get(&self) -> Option<Object> {
        self.inner
            .get()
            .await
            .map_err(|e| error!("Failed to get connection from pool: {}", e))
            .ok()
    }

    /// Returns true if there is a working database connection.
    pub async fn ping(&self) -> bool {
        if let Some(db) = self.get().await {
            if let Ok(row) = db.query_one("SELECT 42;", &[]).await {
                if let Ok(res) = row.try_get::<_, i32>(0) {
                    return res == 42;
                }
            }
        }
        debug!("Failed to ping the database");
        false
    }
}
