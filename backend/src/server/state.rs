use std::sync::Arc;

use crate::{client::db::Database, config::Config};

use super::{popular::PopularPosts, session::Sessions};

/// Use the slots of this struct for cheap-to-clone objects, e.g. Arc<T>.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Database,
    pub sessions: Sessions,
    pub popular_posts: PopularPosts,
}

impl AppState {
    pub fn new(
        config: &Arc<Config>,
        db: Database,
        sessions: Sessions,
        popular_posts: PopularPosts,
    ) -> Self {
        Self {
            config: config.clone(),
            db,
            sessions,
            popular_posts,
        }
    }
}
