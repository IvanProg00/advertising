use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use std::time::Duration;

type Pool<T> = diesel::r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;

pub fn new(database_url: String) -> PostgresPool {
    let manager = ConnectionManager::new(database_url);

    let p = Pool::builder()
        .connection_timeout(Duration::new(3, 0))
        .build(manager)
        .expect("Failed to create pool");

    let mut conn = p.get().unwrap();
    run_migrations(&mut conn);

    p
}

fn run_migrations(conn: &mut PgConnection) {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
