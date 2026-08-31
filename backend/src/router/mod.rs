mod comment;
mod post;
mod posts;
mod session;
mod user;
mod vote;

use crate::server::state::AppState;

use axum::Router;

pub fn create_router(state: AppState) -> Router {
    let router = Router::new();
    let router = user::add_routers(router);
    let router = session::add_routers(router);
    let router = post::add_routers(router);
    let router = posts::add_routers(router);
    let router = comment::add_routers(router);
    let router = vote::add_routers(router);
    router.with_state(state)
}

// TODO: impl. default body limit middleware
