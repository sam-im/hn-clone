use axum::{Router, routing};

use crate::{
    handler::comment::{get_comment, post_comment},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/comment", routing::post(post_comment))
        .route("/comment/{id}", routing::get(get_comment))
}
