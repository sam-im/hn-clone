use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{
    dto::{
        Validate,
        user::{
            RegisterUserRequest, RegisterUserResponse, UpdateField, UpdateUserRequest, UserResponse,
        },
    },
    error::{AppError, AppResult},
    server::state::AppState,
};

pub async fn register_user(
    state: AppState,
    req: RegisterUserRequest,
) -> AppResult<RegisterUserResponse> {
    req.validate()?;
    let mut db = state.db.get().await?;
    let transaction = db.transaction().await?;

    let statement = transaction
        .prepare_cached("SELECT _id FROM _user WHERE _user._username = $1;")
        .await?;
    let result = transaction.query(&statement, &[&req.username]).await?;
    if !(result.is_empty()) {
        return Err(AppError::ResourceExistsError);
    }

    let statement = transaction
        .prepare_cached("INSERT INTO _user (_username, _password_hash) VALUES ($1, $2);")
        .await?;
    let phc = hash_password(&req.password)?;
    transaction
        .execute(&statement, &[&req.username, &phc])
        .await?;

    let statement = transaction
        .prepare_cached("SELECT _id FROM _user WHERE _username = $1;")
        .await?;
    let result = transaction.query_one(&statement, &[&req.username]).await?;
    let id: i32 = result.get(0);

    transaction.commit().await?;
    Ok(RegisterUserResponse { id })
}

pub async fn retrieve_user(state: AppState, username: &str) -> AppResult<UserResponse> {
    let db = state.db.get().await?;
    let statement = db
        .prepare_cached("SELECT _username, _about, _public_key FROM _user WHERE _username = $1;")
        .await?;

    let rows = db.query(&statement, &[&username]).await?;
    let row = match rows.first() {
        Some(r) => r,
        None => return Err(AppError::ResourceNotFound),
    };

    let username: String = row.get("_username");
    let about: Option<String> = row.get("_about");
    let pubkey: Option<String> = row.get("_public_key");
    let user = UserResponse {
        username,
        about,
        pubkey,
    };
    Ok(user)
}

pub async fn update_user(
    state: AppState,
    token: &str,
    username: &str,
    req: UpdateUserRequest,
) -> AppResult {
    req.validate()?;

    let session = match state.sessions.get_session(token).await? {
        Some(s) => s,
        None => return Err(AppError::AuthError("invalid session token".to_string())),
    };

    let mut db = state.db.get().await?;
    let transaction = db.transaction().await?;

    let statement = transaction
        .prepare_cached("SELECT _username FROM _user WHERE _user_id = $1")
        .await?;
    let rows = transaction.query(&statement, &[&session.user_id]).await?;
    let row = match rows.first() {
        Some(r) => r,
        None => return Err(AppError::ResourceNotFound),
    };

    let session_username: String = row.get("_username");
    if session_username != username {
        return Err(AppError::AuthError(format!(
            "token bearer is not authorized to modify {}",
            username
        )));
    }

    if let UpdateField::Set(password) = req.password {
        let phc = hash_password(&password)?;
        let statement = transaction
            .prepare_cached("UPDATE _user SET _password_hash = $1 WHERE _user_id = $2;")
            .await?;
        transaction
            .execute(&statement, &[&phc, &session.user_id])
            .await?;
    }

    match req.about {
        UpdateField::Unspecified => (),
        UpdateField::Set(about) => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _about = $1 WHERE _user_id = $2;")
                .await?;
            transaction
                .execute(&statement, &[&about, &session.user_id])
                .await?;
        }
        UpdateField::Clear => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _about = NULL WHERE _user_id = $1;")
                .await?;
            transaction.execute(&statement, &[&session.user_id]).await?;
        }
    }

    match req.pubkey {
        UpdateField::Unspecified => (),
        UpdateField::Set(pubkey) => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _public_key = $1 WHERE _user_id = $2;")
                .await?;
            // TODO: consider importing and exporting the key with rpgp crate
            // with the hope of removing unnecessary information.
            transaction
                .execute(&statement, &[&pubkey, &session.user_id])
                .await?;
        }
        UpdateField::Clear => {
            let statement = transaction
                .prepare_cached("UPDATE _user SET _public_key = NULL WHERE _user_id = $1;")
                .await?;
            transaction.execute(&statement, &[&session.user_id]).await?;
        }
    }

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
