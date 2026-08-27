use crate::{
    dto::{
        Validate,
        user::{RegisterUserRequest, UpdateUserRequest, UserResponse, validate_username},
    },
    error::AppResult,
    server::{session::verify_session, state::AppState},
    service::user::{register_user, retrieve_user, update_user},
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::{info, warn};

pub async fn get_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    validate_username(&username)?;

    match retrieve_user(state, &username).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => {
            warn!("Failed to retrieve user: {e:?}");
            Err(e)
        }
    }
}

pub async fn post_user(
    State(state): State<AppState>,
    Json(body): Json<RegisterUserRequest>,
) -> AppResult<(StatusCode, Json<UserResponse>)> {
    body.validate()?;

    match register_user(state, body).await {
        Ok(resp) => {
            info!("registered: {}", resp.username);
            Ok((StatusCode::CREATED, Json(resp)))
        }
        Err(e) => {
            warn!("Failed to register user: {e:?}");
            Err(e)
        }
    }
}

pub async fn patch_user(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Path(username): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> AppResult<StatusCode> {
    token.validate()?;
    let session = verify_session(&state.sessions, token.token()).await?;
    validate_username(&username)?;
    body.validate()?;

    match update_user(state, session, &username, body).await {
        Ok(_) => {
            info!("updated user with username: {}", username);
            Ok(StatusCode::OK)
        }
        Err(e) => {
            warn!("Failed to update user: {e:?}");
            Err(e)
        }
    }
}
