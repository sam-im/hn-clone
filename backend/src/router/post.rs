use axum::{Router, routing};

use crate::{
    handler::post::{get_comments, get_post, post_post},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/post", routing::post(post_post))
        .route("/post/{id}", routing::get(get_post))
        .route("/post/{id}/comments", routing::get(get_comments))
}
