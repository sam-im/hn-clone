use crate::server::state::State;

use axum::Router;

pub fn create_app(state: State) -> Router {
    let router = Router::new();
    router.with_state(state)
}
