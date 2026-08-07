use std::time::Duration;

use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::{
    dto::{
        Validate,
        session::{SessionRequest, SessionResponse, TokenFromRequest},
    },
    error::{AppError, AppResult},
    server::state::AppState,
};

pub async fn create_session(state: AppState, req: SessionRequest) -> AppResult<SessionResponse> {
    req.validate()?;
    let db = state.db.get().await?;
    let statement = db
        .prepare_cached("SELECT _id, _password_hash FROM _user WHERE _username = $1;")
        .await?;

    let rows = db.query(&statement, &[&req.username]).await?;
    let (user_id, phc): (i32, String) = match rows.first() {
        Some(row) => (row.get("_id"), row.get("_password_hash")),
        None => return Err(AppError::ResourceNotFound),
    };

    let phc = PasswordHash::new(&phc)?;
    let argon2 = Argon2::default();
    if let Err(_) = argon2.verify_password(&req.password.as_bytes(), &phc) {
        return Err(AppError::AuthError("incorrect password".to_string()));
    }

    let duration = Duration::from_mins(req.duration.into());
    let (token, session) = state.sessions.create_session(user_id, duration).await?;

    Ok(SessionResponse::try_from((token, session))?)
}

pub async fn remove_session(state: AppState, token: TokenFromRequest) -> AppResult {
    token.validate()?;
    match state.sessions.delete_session(&token.token).await? {
        Some(_) => Ok(()),
        None => Err(AppError::ResourceNotFound),
    }
}

pub async fn retrieve_session(
    state: AppState,
    token: TokenFromRequest,
) -> AppResult<SessionResponse> {
    token.validate()?;
    let session = match state.sessions.get_session(&token.token).await? {
        Some(s) => s,
        None => return Err(AppError::ResourceNotFound),
    };
    Ok(SessionResponse::try_from((token.token, session))?)
}
