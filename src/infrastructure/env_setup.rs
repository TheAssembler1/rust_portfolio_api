use std::sync::OnceLock;

use anyhow::bail;
use dotenv::dotenv;
use serde::Deserialize;

pub static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();
pub static DATABASE_CONFIG: OnceLock<DatabaseConfig> = OnceLock::new();

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

pub fn get_server_config() -> anyhow::Result<&'static ServerConfig> {
    match SERVER_CONFIG.get() {
        Some(server_config) => Ok(server_config),
        None => bail!("expected SERVER_CONFIG to be set"),
    }
}

pub fn get_database_config() -> anyhow::Result<&'static DatabaseConfig> {
    match DATABASE_CONFIG.get() {
        Some(database_config) => Ok(database_config),
        None => bail!("expected DATABASE_CONFIG to be set"),
    }
}

pub fn init_env() -> anyhow::Result<()> {
    dotenv().ok();

    let server_config = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;
    if SERVER_CONFIG.set(server_config).is_err() {
        bail!("failed to set SERVER_CONFIG");
    }

    let database_config = envy::prefixed("DATABASE_").from_env::<DatabaseConfig>()?;
    if DATABASE_CONFIG.set(database_config).is_err() {
        bail!("failed to set DATABASE_CONFIG");
    }

    Ok(())
}
