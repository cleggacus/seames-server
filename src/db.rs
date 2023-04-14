use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PoolError, PooledConnection};
use diesel::r2d2::Pool;
use dotenv::dotenv;
use std::env;

pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<DBPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> DBPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined");

    init_pool(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

