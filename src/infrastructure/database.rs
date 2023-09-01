use crate::infrastructure::env_setup;
use diesel_async::{AsyncConnection, AsyncMysqlConnection};

pub async fn establish_connection() -> AsyncMysqlConnection {
    let database_config = env_setup::get_database_config()
        .unwrap_or_else(|_| panic!("failed to get DATABASE_CONFIG"));

    AsyncMysqlConnection::establish(&database_config.url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_config.url))
}
