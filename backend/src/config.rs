use std::env::var;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use tracing::error;

pub const USERNAME_MIN_LEN: usize = 4;
pub const USERNAME_MAX_LEN: usize = 36;
pub const PASSWORD_MIN_LEN: usize = 8;
pub const PASSWORD_MAX_LEN: usize = 64;
pub const PUBKEY_MAX_LEN: usize = 64 * 1024;
/// Allowed length of the about section in user profiles.
pub const ABOUT_MAX_LEN: usize = 256;
/// Allowed durations for session expiry in minutes.
pub const SESSION_DURATIONS: &[u32] = &[60, 8 * 60, 24 * 60, 7 * 24 * 60];
/// Length of the randonmly generated token length.
pub const SESSION_TOKEN_LEN: usize = 64;
pub const SESSION_CLEANUP_INTERVAL: Duration = Duration::from_hours(1);
pub const SESSION_CHANNEL_CAPACITY: usize = 256;
pub const SESSION_CHANNEL_TIMEOUT: Duration = Duration::from_secs(10);
pub const POST_TITLE_LEN: usize = 256;
pub const POST_CONTENT_LEN: usize = 2048;

#[derive(Clone)]
pub struct Config {
    pub server_addr: IpAddr,
    pub server_port: u16,
    pub db_addr: IpAddr,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_pool_size: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let try_env_var = |key: &str, default: Option<&str>| -> String {
            match var(key) {
                Ok(v) => v,
                Err(_) => match default {
                    Some(v) => v.to_owned(),
                    None => {
                        error!("A variable is not set and no default was provided: {}", key);
                        std::process::exit(1);
                    }
                },
            }
        };
        let server_addr = IpAddr::from_str(try_env_var("SERVER_ADDR", Some("127.0.0.1")).as_ref())?;
        let server_port = u16::from_str(try_env_var("SERVER_PORT", Some("3000")).as_ref())?;
        let db_addr = IpAddr::from_str(try_env_var("DB_ADDR", Some("127.0.0.1")).as_ref())?;
        let db_name = try_env_var("DB_NAME", None);
        let db_user = try_env_var("DB_USER", None);
        let db_pass = try_env_var("DB_PASS", None);
        let db_pool_size = usize::from_str(try_env_var("DB_POOL_SIZE", Some("16")).as_ref())?;
        Ok(Self {
            server_addr,
            server_port,
            db_addr,
            db_name,
            db_user,
            db_pass,
            db_pool_size,
        })
    }
}
