use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub enum DatabaseKind {
    Main,
}

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection(db_kind: DatabaseKind) -> DbPool {
    dotenv().ok();

    let database_url = match db_kind {
        DatabaseKind::Main => env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    init_pool(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
