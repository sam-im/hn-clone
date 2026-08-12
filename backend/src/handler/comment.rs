use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::info;

use crate::{
    dto::{
        Validate,
        comment::{CommentResponse, NewCommentRequest},
    },
    error::AppResult,
    server::{session::verify_session, state::AppState},
    service::comment::{create_comment, retrieve_comment},
};

pub async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<(StatusCode, Json<CommentResponse>)> {
    match retrieve_comment(state, id).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err(e),
    }
}
pub async fn post_comment(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(body): Json<NewCommentRequest>,
) -> AppResult<(StatusCode, Json<CommentResponse>)> {
    token.validate()?;
    let session = verify_session(&state.sessions, token.token()).await?;
    body.validate()?;

    match create_comment(state, session, body).await {
        Ok(res) => {
            info!("{} commented on {}", res.owner, res.parent);
            Ok((StatusCode::CREATED, Json(res)))
        }
        Err(e) => Err(e),
    }
}
