use std::sync::Arc;

use crate::{client::db::Database, config::Config};

use super::session::Sessions;

/// Use the slots of this struct for cheap-to-clone objects, e.g. Arc<T>.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Database,
    pub sessions: Sessions,
}

impl AppState {
    pub fn new(config: &Arc<Config>, db: Database, sessions: Sessions) -> Self {
        Self {
            config: config.clone(),
            db,
            sessions,
        }
    }
}
