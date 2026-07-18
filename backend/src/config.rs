use std::env::var;
use std::net::IpAddr;
use std::str::FromStr;

use tracing::error;

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
        let custom_var = |key: &str, default: Option<&str>| -> String {
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
        let server_addr = IpAddr::from_str(custom_var("SERVER_ADDR", Some("127.0.0.1")).as_ref())?;
        let server_port = u16::from_str(custom_var("SERVER_PORT", Some("3000")).as_ref())?;
        let db_addr = IpAddr::from_str(custom_var("DB_ADDR", Some("127.0.0.1")).as_ref())?;
        let db_name = custom_var("DB_NAME", None).into();
        let db_user = custom_var("DB_USER", None).into();
        let db_pass = custom_var("DB_PASS", None).into();
        let db_pool_size = usize::from_str(custom_var("DB_POOL_SIZE", Some("16")).as_ref())?;
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
