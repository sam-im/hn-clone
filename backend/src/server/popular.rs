use std::{sync::Arc, time::SystemTime};

use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::info;

use crate::{
    client::db::Database,
    config::{
        POPULAR_POSTS_CHANNEL_CAPACITY, POPULAR_POSTS_CHANNEL_TIMEOUT,
        POPULAR_POSTS_UPDATE_INTERVAL,
    },
    dto::{PaginationParams, PaginationResponse, post::PostResponse},
    error::{AppError, AppResult},
    service::posts::popular_posts,
};

#[derive(Clone)]
pub struct PopularPosts {
    tx: mpsc::Sender<PopularPostsMsg>,
    _handle: Arc<JoinHandle<()>>,
}

impl PopularPosts {
    pub fn new(db: Database) -> Result<Self, Box<dyn std::error::Error>> {
        let store = PopularPostsStore::new(db);
        let tx = store.tx.clone();
        let _handle = Arc::new(store.run());
        Ok(Self { tx, _handle })
    }

    pub async fn get(
        &self,
        pagination: &PaginationParams,
    ) -> AppResult<PaginationResponse<PostResponse>> {
        let (tx, rx) = oneshot::channel();
        let msg = PopularPostsMsg::Get(*pagination, tx);
        self.tx
            .send_timeout(msg, POPULAR_POSTS_CHANNEL_TIMEOUT)
            .await
            .map_err(|e| AppError::Session(e.to_string()))?;
        rx.await.map_err(|e| AppError::Session(e.to_string()))
    }
}

enum PopularPostsMsg {
    Get(
        PaginationParams,
        oneshot::Sender<PaginationResponse<PostResponse>>,
    ),
    Set(Vec<PostResponse>),
}

struct PopularPostsStore {
    inner: Vec<PostResponse>,
    rx: mpsc::Receiver<PopularPostsMsg>,
    tx: mpsc::Sender<PopularPostsMsg>,
    db: Database,
    last_update: Option<SystemTime>,
}

impl PopularPostsStore {
    fn new(db: Database) -> Self {
        let inner = Vec::new();
        let (tx, rx) = mpsc::channel(POPULAR_POSTS_CHANNEL_CAPACITY);
        let last_update = None;
        Self {
            inner,
            rx,
            tx,
            db,
            last_update,
        }
    }

    fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            info!("starting event loop for popular posts");
            loop {
                self.check_update();
                match self.rx.recv().await {
                    Some(msg) => match msg {
                        PopularPostsMsg::Get(pagination, tx) => {
                            let posts = self
                                .inner
                                .iter()
                                .skip(pagination.offset as usize)
                                .take(pagination.limit as usize)
                                .map(|p| p.to_owned())
                                .collect::<Vec<PostResponse>>();
                            let offset = if posts.len() == pagination.limit as usize {
                                Some(pagination.offset as usize + posts.len())
                            } else {
                                None
                            };
                            let res = PaginationResponse {
                                offset,
                                data: posts,
                            };
                            let _ = tx.send(res);
                        }
                        PopularPostsMsg::Set(posts) => self.inner = posts,
                    },
                    None => {
                        // close() is called on the channel
                        info!("stopping event loop for popular posts");
                        break;
                    }
                }
            }
        })
    }

    fn check_update(&mut self) {
        if self.last_update.is_none()
            || self
                .last_update
                .is_some_and(|t| t + POPULAR_POSTS_UPDATE_INTERVAL < SystemTime::now())
        {
            self.last_update = Some(SystemTime::now());
            let db = self.db.clone();
            let tx = self.tx.clone();

            tokio::spawn(async move {
                let posts = popular_posts(
                    db,
                    &PaginationParams {
                        offset: 0,
                        limit: 100,
                    },
                )
                .await
                .unwrap();
                let _ = tx.send(PopularPostsMsg::Set(posts.data)).await;
                info!("updated popular posts list");
            });
        }
    }
}
