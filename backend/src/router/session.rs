use axum::{Router, routing};

use crate::{
    handler::session::{delete_session, get_session, post_session},
    server::state::AppState,
};

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/session", routing::post(post_session))
        .route("/session/{token}", routing::get(get_session))
        .route("/session/{token}", routing::delete(delete_session))
}
