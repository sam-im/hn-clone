use axum::extract::State;

use crate::server::state::AppState;

pub async fn post_post(State(_state): State<AppState>) {}
pub async fn get_post(State(_state): State<AppState>) {}
pub async fn get_posts(State(_state): State<AppState>) {}
