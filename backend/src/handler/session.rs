use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::warn;

use crate::{
    dto::{
        Validate,
        session::{SessionRequest, SessionResponse, TokenFromRequest},
    },
    error::AppResult,
    server::state::AppState,
    service::session::{create_session, remove_session, retrieve_session},
};

pub async fn post_session(
    State(state): State<AppState>,
    Json(body): Json<SessionRequest>,
) -> AppResult<(StatusCode, Json<SessionResponse>)> {
    body.validate()?;

    match create_session(state, body).await {
        Ok(session) => Ok((StatusCode::CREATED, Json(session))),
        Err(e) => {
            warn!("failed to create session: {e}");
            Err(e)
        }
    }
}

pub async fn get_session(
    State(state): State<AppState>,
    Path(token): Path<TokenFromRequest>,
) -> AppResult<(StatusCode, Json<SessionResponse>)> {
    token.validate()?;

    match retrieve_session(state, token).await {
        Ok(resp) => Ok((StatusCode::OK, Json(resp))),
        Err(e) => Err(e),
    }
}

pub async fn delete_session(
    State(state): State<AppState>,
    Path(token): Path<TokenFromRequest>,
) -> AppResult<StatusCode> {
    token.validate()?;

    match remove_session(state, token).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}
