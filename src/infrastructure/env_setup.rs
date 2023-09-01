use std::sync::OnceLock;

use anyhow::bail;
use dotenv::dotenv;
use serde::Deserialize;

pub static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();

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

pub fn init_env() -> anyhow::Result<()> {
    dotenv().ok();

    let server_config = envy::prefixed("SERVER_CONFIG_").from_env::<ServerConfig>()?;

    if SERVER_CONFIG.set(server_config).is_err() {
        bail!("failed to set SERVER_CONFIG");
    }

    Ok(())
}
