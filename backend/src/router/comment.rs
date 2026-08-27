use axum::{Router, routing};

use crate::{
    handler::comment::{get_comment, get_replies, post_comment},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/comment", routing::post(post_comment))
        .route("/comment/{id}", routing::get(get_comment))
        .route("/comment/{id}/replies", routing::get(get_replies))
}
