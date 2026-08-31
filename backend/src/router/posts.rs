use axum::{Router, routing};

use crate::{handler::posts::get_posts, server::state::AppState};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router.route("/posts", routing::get(get_posts))
}
