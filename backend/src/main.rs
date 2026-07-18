mod client;
mod config;
mod router;
mod server;
mod service;

use tracing::{error, info};

use crate::client::db::Database;
use crate::config::Config;
use crate::router::create_app;
use crate::server::state::State;

use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(Config::from_env()?);
    let db = Database::new(&config)?;
    let state = State::new(&config, db);

    let app = create_app(state);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", &config.server_addr, &config.server_port))
            .await?;
    info!(
        "Listening on {}:{}",
        &config.server_addr, &config.server_port
    );
    match axum::serve(listener, app).await {
        Ok(_) => info!("Exited"),
        Err(e) => error!("Exited with error: {}", e),
    }

    Ok(())
}
