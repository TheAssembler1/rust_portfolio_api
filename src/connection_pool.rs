use mysql::{Pool, PooledConn};
use std::sync::OnceLock;
pub static CONNECTION_POOL: OnceLock<ConnectionPool> = OnceLock::new();

#[derive(Debug)]
pub struct ConnectionPool {
    pub pool: Pool,
}

impl ConnectionPool {
    pub fn global() -> &'static ConnectionPool {
        CONNECTION_POOL
            .get()
            .expect("CONNECTION_POOL is not initialized!")
    }

    pub fn init(db_url: String) -> Pool {
        let pool = match Pool::new(&db_url[..]) {
            Ok(pool) => {
                println!("Connection to database successful!");
                pool
            }
            Err(error) => panic!("{:#?}", error),
        };

        pool
    }

    pub fn get_conn() -> PooledConn {
        let conn_pool = &ConnectionPool::global().pool;
        let conn = conn_pool.get_conn().unwrap();
        conn
    }
}
