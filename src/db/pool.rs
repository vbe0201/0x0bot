use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> PgPool {
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(connection_manager).expect("Failed to create Postgres connection pool!")
}
