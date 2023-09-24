use std::{sync::OnceLock, path::Path, fs::File, io::{Read, Write}, any};

use anyhow::bail;
use dotenv::dotenv;
use jwt_simple::prelude::HS256Key;
use log::{info, error};
use serde::Deserialize;

pub static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();
pub static DATABASE_CONFIG: OnceLock<DatabaseConfig> = OnceLock::new();
pub static JWT_KEY: OnceLock<HS256Key> = OnceLock::new();
pub static JWT_CONFIG: OnceLock<JwtConfig> = OnceLock::new();

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct JwtConfig {
    pub key_path: String,
    pub expiration_in_hours: usize,   
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

pub fn get_jwt_key() -> anyhow::Result<&'static HS256Key> {
    match JWT_KEY.get() {
        Some(jwt_key) => Ok(jwt_key),
        None => bail!("expected JWT_KEY to be set"),
    }
}

pub fn get_jwt_config() -> anyhow::Result<&'static JwtConfig> {
    match JWT_CONFIG.get() {
        Some(jwt_config) => Ok(jwt_config),
        None => bail!("expected JWT_CONFIG to be set"),
    }
}

pub fn init_env() -> anyhow::Result<()> {
    dotenv().ok();

    let jwt_config = envy::prefixed("JWT_").from_env::<JwtConfig>()?;
    if JWT_CONFIG.set(jwt_config).is_err() {
        bail!("failed to set JWT_CONFIG");
    }

    let server_config = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;
    if SERVER_CONFIG.set(server_config).is_err() {
        bail!("failed to set SERVER_CONFIG");
    }

    let database_config = envy::prefixed("DATABASE_").from_env::<DatabaseConfig>()?;
    if DATABASE_CONFIG.set(database_config).is_err() {
        bail!("failed to set DATABASE_CONFIG");
    }

    init_jwt_key()?;

    Ok(())
}

pub fn init_jwt_key() -> anyhow::Result<()> {
    info!("checking for jwt private key");

    let jwt_config = get_jwt_config()?;
    let jwt_private_key_file_path = Path::new(&jwt_config.key_path);

    let key: HS256Key;
    if jwt_private_key_file_path.exists() {
        info!("jwt private key found");

        let mut file = File::open(&jwt_config.key_path)?;
        let mut buffer = Vec::new();
        let _file_size = file.read_to_end(&mut buffer)?;

        key = HS256Key::from_bytes(&buffer);
    } else {
        info!("jwt private key not found");
        info!("creating new jwt private key and writing to {}", &jwt_config.key_path);

        key = HS256Key::generate();

        let mut file = File::create(&jwt_config.key_path)?;
        file.write_all(&key.to_bytes()[..])?;
    }

    if let Err(err) = JWT_KEY.set(key) {
        bail!("failed to set JWT_KEY");
    }

    Ok(())
}
