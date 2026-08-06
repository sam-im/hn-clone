use rand::{
    RngExt, SeedableRng,
    distr::Alphanumeric,
    rngs::{StdRng, SysRng},
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::{error, info, warn};

use crate::error::{AppError, AppResult};

const TOKEN_LEN: usize = 64;
const CLEANUP_INTERVAL: Duration = Duration::from_hours(1);
const CHANNEL_CAPACITY: usize = 256;
const CHANNEL_TIMEOUT: Duration = Duration::from_secs(10);

/// Session Data
#[derive(Clone)]
pub struct Session {
    pub user_id: u32,
    pub issued_at: Instant,
    pub expires_at: Instant,
}

impl Session {
    fn new(user_id: u32, duration: Duration) -> Self {
        let now = std::time::Instant::now();
        Self {
            user_id,
            issued_at: now,
            expires_at: now + duration,
        }
    }
}

enum SessionMsg {
    New(u32, Duration, oneshot::Sender<String>),
    Get(String, oneshot::Sender<Option<Session>>),
    Del(String, oneshot::Sender<Option<Session>>),
}

struct SessionStore {
    inner: HashMap<String, Session>,
    rng: StdRng,
    rx: mpsc::Receiver<SessionMsg>,
    last_cleanup: Instant,
}

impl SessionStore {
    fn new(rx: mpsc::Receiver<SessionMsg>) -> Result<JoinHandle<()>, Box<dyn std::error::Error>> {
        let inner = HashMap::new();
        let rng = StdRng::try_from_rng(&mut SysRng)?;
        let last_cleanup = Instant::now();
        let store = Self {
            inner,
            rng,
            rx,
            last_cleanup,
        };
        let handle = tokio::spawn(store.event_loop());
        Ok(handle)
    }

    fn cleanup(&mut self) {
        let now = Instant::now();
        let mut count = 0;
        if self.last_cleanup + CLEANUP_INTERVAL < now {
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
            self.last_cleanup = Instant::now();
            info!("cleaned up {} sessions", count);
        }
    }

    fn generate_token(&mut self) -> String {
        (&mut self.rng)
            .sample_iter(&Alphanumeric)
            .take(TOKEN_LEN)
            .map(char::from)
            .collect()
    }

    async fn event_loop(mut self) {
        loop {
            match self.rx.recv().await {
                Some(msg) => {
                    match msg {
                        SessionMsg::New(user_id, duration, tx) => {
                            // potentially remove expired entries
                            self.cleanup();

                            // remove existing session, if any
                            if let Some(key) = self
                                .inner
                                .iter()
                                .find(|e| e.1.user_id == user_id)
                                .map(|e| e.0.to_owned())
                            {
                                self.inner.remove(&key);
                            }

                            // create and return the new token
                            let session = Session::new(user_id, duration);
                            let token = self.generate_token();
                            self.inner.insert(token.clone(), session);
                            let _ = tx.send(token);
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

#[derive(Clone)]
pub struct Sessions {
    tx: mpsc::Sender<SessionMsg>,
    handle: Arc<JoinHandle<()>>,
}

impl Sessions {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        let handle = Arc::new(SessionStore::new(rx)?);
        Ok(Self { tx, handle })
    }

    pub async fn create_session(&self, user_id: u32, duration: Duration) -> AppResult<String> {
        let (tx, rx) = oneshot::channel();
        let msg = SessionMsg::New(user_id, duration, tx);
        self.tx
            .send_timeout(msg, CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::SessionError(e.to_string()))?;
        rx.await.map_err(|e| AppError::SessionError(e.to_string()))
    }

    pub async fn get_session(&self, token: &str) -> AppResult<Option<Session>> {
        let (tx, rx) = oneshot::channel();
        let msg = SessionMsg::Get(token.to_string(), tx);
        self.tx
            .send_timeout(msg, CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::SessionError(e.to_string()))?;
        rx.await.map_err(|e| AppError::SessionError(e.to_string()))
    }

    pub async fn delete_session(&self, token: &str) -> AppResult<Option<Session>> {
        let (tx, rx) = oneshot::channel();
        let msg = SessionMsg::Del(token.to_string(), tx);
        self.tx
            .send_timeout(msg, CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::SessionError(e.to_string()))?;
        rx.await.map_err(|e| AppError::SessionError(e.to_string()))
    }
}
