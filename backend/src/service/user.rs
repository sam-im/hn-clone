use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{
    dto::{
        OptionalField,
        user::{RegisterUserRequest, UpdateUserRequest, UserResponse},
    },
    error::{AppError, AppResult},
    server::{session::Session, state::AppState},
};

pub async fn register_user(state: AppState, req: RegisterUserRequest) -> AppResult<UserResponse> {
    let mut db = state.db.get().await?;
    let transaction = db.transaction().await?;

    let check_stmt = transaction
        .prepare_cached("SELECT _id FROM _user WHERE _user._username = $1;")
        .await?;
    let result = transaction.query(&check_stmt, &[&req.username]).await?;
    if !(result.is_empty()) {
        return Err(AppError::ResourceExists);
    }

    let insert_stmt = transaction
        .prepare_cached("INSERT INTO _user (_username, _password_hash) VALUES ($1, $2);")
        .await?;
    let phc = hash_password(&req.password)?;
    transaction
        .execute(&insert_stmt, &[&req.username, &phc])
        .await?;
    transaction.commit().await?;

    let query_stmt = db
        .prepare_cached("SELECT _username, _about, _public_key FROM _user WHERE _username = $1;")
        .await?;
    let row = db.query_one(&query_stmt, &[&req.username]).await?;

    Ok(UserResponse::from(&row))
}

pub async fn retrieve_user(state: AppState, username: &str) -> AppResult<UserResponse> {
    let db = state.db.get().await?;
    let statement = db
        .prepare_cached("SELECT _username, _about, _public_key FROM _user WHERE _username = $1;")
        .await?;

    let rows = db.query(&statement, &[&username]).await?;
    match rows.first() {
        Some(r) => Ok(UserResponse::from(r)),
        None => Err(AppError::ResourceNotFound),
    }
}

pub async fn update_user(
    state: AppState,
    session: Session,
    username: &str,
    req: UpdateUserRequest,
) -> AppResult {
    let mut db = state.db.get().await?;
    let transaction = db.transaction().await?;

    let statement = transaction
        .prepare_cached("SELECT _username FROM _user WHERE _id = $1")
        .await?;
    let rows = transaction.query(&statement, &[&session.user_id]).await?;
    let session_username: String = match rows.first() {
        Some(r) => r.get("_username"),
        None => {
            return Err(AppError::Auth(format!(
                "user_id {} no longer exists",
                session.user_id
            )));
        }
    };

    if session_username != username {
        return Err(AppError::Auth(format!(
            " {} is not authorized to modify {}",
            session.user_id, username
        )));
    }

    if let OptionalField::Set(password) = req.password {
        let phc = hash_password(&password)?;
        let statement = transaction
            .prepare_cached("UPDATE _user SET _password_hash = $1 WHERE _id = $2;")
            .await?;
        transaction
            .execute(&statement, &[&phc, &session.user_id])
            .await?;
    }

    match req.about {
        OptionalField::Unspecified => (),
        OptionalField::Set(about) => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _about = $1 WHERE _id = $2;")
                .await?;
            transaction
                .execute(&statement, &[&about, &session.user_id])
                .await?;
        }
        OptionalField::Clear => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _about = NULL WHERE _id = $1;")
                .await?;
            transaction.execute(&statement, &[&session.user_id]).await?;
        }
    }

    match req.pubkey {
        OptionalField::Unspecified => (),
        OptionalField::Set(pubkey) => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _public_key = $1 WHERE _id = $2;")
                .await?;
            // TODO: consider importing and exporting the key with rpgp crate
            // with the hope of removing unnecessary information.
            transaction
                .execute(&statement, &[&pubkey, &session.user_id])
                .await?;
        }
        OptionalField::Clear => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _public_key = NULL WHERE _id = $1;")
                .await?;
            transaction.execute(&statement, &[&session.user_id]).await?;
        }
    }
    transaction.commit().await?;

    Ok(())
}

fn hash_password(password: &str) -> AppResult<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(OsRng);
    let phc = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(phc)
}
