use tokio_postgres::error::SqlState;
use tracing::error;

use crate::{
    error::{AppError, AppResult},
    server::{session::Session, state::AppState},
};

pub async fn add_vote(state: AppState, session: Session, item_id: i32) -> AppResult {
    let db = state.db.get().await?;
    let stmt = db
        .prepare_cached("INSERT INTO _upvote (_item, _user) VALUES ($1, $2);")
        .await?;
    match db.execute(&stmt, &[&item_id, &session.user_id]).await {
        Ok(_) => Ok(()),
        Err(err) => {
            if let Some(db_err) = err.as_db_error() {
                let app_err = match *db_err.code() {
                    SqlState::UNIQUE_VIOLATION => AppError::ResourceExists,
                    SqlState::FOREIGN_KEY_VIOLATION => AppError::ResourceNotFound,
                    _ => AppError::Database(err),
                };
                return Err(app_err);
            }
            Err(AppError::Database(err))
        }
    }
}

pub async fn remove_vote(state: AppState, session: Session, item_id: i32) -> AppResult {
    let db = state.db.get().await?;
    let stmt = db
        .prepare_cached("DELETE FROM _upvote WHERE _item = $1 AND _user = $2;")
        .await?;
    match db.execute(&stmt, &[&item_id, &session.user_id]).await {
        Ok(n) => match n {
            0 => Err(AppError::ResourceNotFound),
            1 => Ok(()),
            _ => {
                error!("removed {} upvotes with a single request", n);
                Ok(())
            }
        },
        Err(e) => Err(AppError::Database(e)),
    }
}
