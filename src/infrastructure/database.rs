use crate::infrastructure::env_setup;
use diesel::{Connection, MysqlConnection};

pub fn establish_connection() -> MysqlConnection {
    let database_config =
        env_setup::get_database_config().unwrap_or_else(|_| panic!("failed to get DATABASE_CONFIG"));

    MysqlConnection::establish(&database_config.url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_config.url))
}
