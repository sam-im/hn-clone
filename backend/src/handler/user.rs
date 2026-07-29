use crate::server::state::AppState;

use axum::extract::State;

pub async fn get_user(State(_state): State<AppState>) {}
pub async fn post_user(State(_state): State<AppState>) {}
pub async fn put_user(State(_state): State<AppState>) {}

