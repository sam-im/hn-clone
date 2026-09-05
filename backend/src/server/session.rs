use rand::{
    RngExt, SeedableRng,
    distr::Alphanumeric,
    rngs::{StdRng, SysRng},
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::{
    sync::{Mutex, mpsc, oneshot},
    task::JoinHandle,
};
use tracing::info;

use crate::{
    config::{
        SESSION_CHANNEL_CAPACITY, SESSION_CHANNEL_TIMEOUT, SESSION_CLEANUP_INTERVAL,
        SESSION_TOKEN_LEN,
    },
    error::{AppError, AppResult},
};

/// Returns an error if the provided `token` doesn't exists or is expired,
/// otherwise returns a `Session`.
pub async fn verify_session(sessions: &Sessions, token: &str) -> AppResult<Session> {
    let session = match sessions.get_session(token).await? {
        Some(s) => {
            if s.is_expired() {
                return Err(AppError::Auth("expired token".to_string()));
            }
            s
        }
        None => return Err(AppError::Auth("invalid token".to_string())),
    };
    Ok(session)
}

#[derive(Clone)]
pub struct Sessions {
    tx: mpsc::Sender<SessionMsg>,
    _handle: Arc<JoinHandle<()>>,
}

impl Sessions {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let store = SessionStore::new()?;
        let tx = store.tx.clone();
        let _handle = Arc::new(store.run());
        Ok(Self { tx, _handle })
    }

    /// For a given user_id and duration create and return the token and the
    /// associated `Session` object.
    pub async fn create_session(
        &self,
        user_id: i32,
        duration: Duration,
    ) -> AppResult<(String, Session)> {
        let (tx, rx) = oneshot::channel();
        let msg = SessionMsg::New(user_id, duration, tx);
        self.tx
            .send_timeout(msg, SESSION_CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::Session(e.to_string()))?;
        rx.await.map_err(|e| AppError::Session(e.to_string()))
    }

    /// Find and return the associated `Session` object for `token`, if it exists.
    pub async fn get_session(&self, token: &str) -> AppResult<Option<Session>> {
        let (tx, rx) = oneshot::channel();
        let msg = SessionMsg::Get(token.to_string(), tx);
        self.tx
            .send_timeout(msg, SESSION_CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::Session(e.to_string()))?;
        rx.await.map_err(|e| AppError::Session(e.to_string()))
    }

    /// Remove the `Session` associated with `token`.
    pub async fn delete_session(&self, token: &str) -> AppResult<Option<Session>> {
        let (tx, rx) = oneshot::channel();
        let msg = SessionMsg::Del(token.to_string(), tx);
        self.tx
            .send_timeout(msg, SESSION_CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::Session(e.to_string()))?;
        rx.await.map_err(|e| AppError::Session(e.to_string()))
    }
}

/// Session Data
#[derive(Clone)]
pub struct Session {
    pub user_id: i32,
    pub issued_at: SystemTime,
    pub expires_at: SystemTime,
}

impl Session {
    fn new(user_id: i32, duration: Duration) -> Self {
        let now = std::time::SystemTime::now();
        Self {
            user_id,
            issued_at: now,
            expires_at: now + duration,
        }
    }
    /// Returns true if the session has expired.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now();
        now > self.expires_at
    }
}

enum SessionMsg {
    New(i32, Duration, oneshot::Sender<(String, Session)>),
    Insert((String, Session), oneshot::Sender<(String, Session)>),
    Get(String, oneshot::Sender<Option<Session>>),
    Del(String, oneshot::Sender<Option<Session>>),
}

struct SessionStore {
    inner: HashMap<String, Session>,
    tx: mpsc::Sender<SessionMsg>,
    rx: mpsc::Receiver<SessionMsg>,
    rng: Arc<Mutex<StdRng>>,
    last_cleanup: SystemTime,
}

impl SessionStore {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let inner = HashMap::new();
        let (tx, rx) = mpsc::channel(SESSION_CHANNEL_CAPACITY);
        let rng = Arc::new(Mutex::new(StdRng::try_from_rng(&mut SysRng)?));
        let last_cleanup = SystemTime::now();

        Ok(Self {
            inner,
            tx,
            rx,
            rng,
            last_cleanup,
        })
    }

    fn run(self) -> JoinHandle<()> {
        tokio::spawn(self.event_loop())
    }

    fn cleanup_expired(&mut self) {
        let now = SystemTime::now();
        let mut count = 0;
        if self.last_cleanup + SESSION_CLEANUP_INTERVAL < now {
            self.inner = self
                .inner
                .iter()
                .filter_map(|(k, v)| {
                    if v.expires_at > now {
                        Some((k.to_owned(), v.to_owned()))
                    } else {
                        count += 1;
                        None
                    }
                })
                .collect();
            self.last_cleanup = SystemTime::now();
            info!("cleaned up {} sessions", count);
        }
    }

    async fn event_loop(mut self) {
        info!("SessionStore has started.");
        loop {
            match self.rx.recv().await {
                Some(msg) => {
                    match msg {
                        SessionMsg::New(user_id, duration, tx) => {
                            // potentially remove expired entries
                            self.cleanup_expired();

                            // remove existing session, if any
                            if let Some(key) = self
                                .inner
                                .iter()
                                .find(|e| e.1.user_id == user_id)
                                .map(|e| e.0.to_owned())
                            {
                                self.inner.remove(&key);
                            }

                            // move token generation to a blocking thread to
                            // avoid stalling the event-loop and/or async runtime
                            let self_tx = self.tx.clone();
                            let rng = self.rng.clone();
                            tokio::task::spawn_blocking(move || {
                                let session = Session::new(user_id, duration);
                                let token = rng
                                    .blocking_lock()
                                    .sample_iter(&Alphanumeric)
                                    .take(SESSION_TOKEN_LEN)
                                    .map(char::from)
                                    .collect();

                                let msg = SessionMsg::Insert((token, session), tx);
                                let _ = self_tx.blocking_send(msg);
                            });
                        }
                        SessionMsg::Insert((token, session), tx) => {
                            self.inner.insert(token.clone(), session.clone());
                            let _ = tx.send((token, session));
                        }
                        SessionMsg::Get(token, tx) => {
                            let _ = tx.send(self.inner.get(&token).map(|s| s.to_owned()));
                        }

                        SessionMsg::Del(token, tx) => {
                            let _ = tx.send(self.inner.remove(&token));
                        }
                    }
                }
                None => {
                    // This means that either close() is called or all senders have been dropped.
                    // You may want to save sessions to a file in /tmp before exiting the loop,
                    // and restore at first startup.
                    info!("SessionStore is stopping.");
                    break;
                }
            }
        }
    }
}
