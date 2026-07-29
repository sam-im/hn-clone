use axum::extract::State;

use crate::server::state::AppState;

pub async fn post_vote(State(_state): State<AppState>) {}
pub async fn delete_vote(State(_state): State<AppState>) {}
