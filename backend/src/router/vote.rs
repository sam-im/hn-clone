use axum::{Router, routing};

use crate::{
    handler::vote::{delete_vote, post_vote},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/vote/{id}", routing::get(post_vote))
        .route("/vote/{id}", routing::delete(delete_vote))
}
