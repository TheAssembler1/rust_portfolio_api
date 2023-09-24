use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncMysqlConnection;

pub type DbPool = Pool<AsyncMysqlConnection>;