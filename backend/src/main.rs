mod client;
mod config;
mod dto;
mod error;
mod handler;
mod router;
mod server;
mod service;

use server::popular::PopularPosts;
use server::session::Sessions;
use tracing::info;

use crate::client::db::Database;
use crate::config::Config;
use crate::router::create_router;
use crate::server::state::AppState;

use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(Config::from_env()?);
    let db = Database::new(&config)?;
    db.get().await?.check_connection().await?;
    let sessions = Sessions::new()?;
    let popular_posts = PopularPosts::new(db.clone())?;
    let state = AppState::new(&config, db, sessions, popular_posts);

    let router = create_router(state);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", &config.server_addr, &config.server_port))
            .await?;
    info!(
        "Listening on {}:{}",
        &config.server_addr, &config.server_port
    );
    axum::serve(listener, router).await?;
    Ok(())
}
