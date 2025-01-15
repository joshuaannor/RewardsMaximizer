// Backend/RewardsMaxAPI/src/web/db.rs
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct DbManager {
    pool: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

impl DbManager {
    pub fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager)?;

        Ok(DbManager {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>> {
        self.pool.clone()
    }
}
