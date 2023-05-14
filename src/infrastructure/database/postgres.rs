use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use std::time::Duration;

type Pool<T> = diesel::r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;

pub fn new(database_url: String) -> PostgresPool {
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .connection_timeout(Duration::new(3, 0))
        .build(manager)
        .expect("Failed to create pool")
}
