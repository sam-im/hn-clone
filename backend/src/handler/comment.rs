use axum::extract::State;

use crate::server::state::AppState;

pub async fn get_comment(State(_state): State<AppState>) {}
pub async fn post_comment(State(_state): State<AppState>) {}
