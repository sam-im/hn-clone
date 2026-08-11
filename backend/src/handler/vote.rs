use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::warn;

use crate::{
    dto::Validate,
    error::AppResult,
    server::state::AppState,
    service::{
        session::verify_session,
        vote::{add_vote, remove_vote},
    },
};

pub async fn post_vote(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Path(item_id): Path<i32>,
) -> AppResult<StatusCode> {
    token.validate()?;
    let session = verify_session(&state, token.token()).await?;

    match add_vote(state, session, item_id).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            warn!("{e:?}");
            Err(e)
        }
    }
}
pub async fn delete_vote(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Path(item_id): Path<i32>,
) -> AppResult<StatusCode> {
    token.validate()?;
    let session = verify_session(&state, token.token()).await?;

    match remove_vote(state, session, item_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            warn!("{e:?}");
            Err(e)
        }
    }
}
