use axum::{Router, routing};

use crate::{
    handler::user::{get_user, patch_user, post_user},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/user", routing::post(post_user))
        .route("/user/{username}", routing::get(get_user))
        .route("/user/{username}", routing::patch(patch_user))
}
