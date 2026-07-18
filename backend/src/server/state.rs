use std::sync::Arc;

use crate::{client::db::Database, config::Config};

/// Use the slots of this struct for cheap-to-clone objects, e.g. Arc<T>.
#[derive(Clone)]
pub struct State {
    config: Arc<Config>,
    db: Database,
}

impl State {
    pub fn new(config: &Arc<Config>, db: Database) -> Self {
        Self {
            config: config.clone(),
            db,
        }
    }
}
