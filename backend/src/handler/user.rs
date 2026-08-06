use crate::{
    dto::user::{RegisterUserRequest, RegisterUserResponse, UpdateUserRequest, UserResponse},
    error::AppResult,
    server::state::AppState,
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
) -> AppResult<(StatusCode, Json<RegisterUserResponse>)> {
    match register_user(state, body).await {
        Ok(response) => {
            info!("registered a new user with id: {}", response.id);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            warn!("Failed to register user: {e:?}");
            Err(e)
        }
    }
}

pub async fn patch_user(
    State(state): State<AppState>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    Path(username): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> AppResult<StatusCode> {
    match update_user(state, bearer.token(), &username, body).await {
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
