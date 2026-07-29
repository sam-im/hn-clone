use axum::{Router, routing};

use crate::{
    handler::post::{get_post, get_posts, post_post},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/post", routing::post(post_post))
        .route("/posts", routing::get(get_posts))
        .route("/post/{id}", routing::get(get_post))
}
