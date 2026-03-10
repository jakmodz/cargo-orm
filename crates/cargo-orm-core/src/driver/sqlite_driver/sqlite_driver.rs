use sqlx::SqlitePool;

use crate::driver::{connection_pool::ConnectionPool, sql_driver::SqlDriver};

pub struct SqliteDriver {
    pool: SqlitePool,
}

impl SqlDriver for SqliteDriver {
    type Pool = SqlitePool;

    async fn new(
        config: <Self::Pool as crate::driver::connection_pool::ConnectionPool>::Config,
    ) -> Result<Self, crate::error::CargoOrmError> {
        let pool = SqlitePool::new_pool(config).await?;
        Ok(Self { pool })
    }
    fn pool(&self) -> &Self::Pool {
        &self.pool
    }
    async fn transaction(
        &self,
    ) -> Result<crate::driver::transaction::Transaction<Self::Pool>, crate::error::CargoOrmError>
    {
        let tx = self.pool.begin_transaction().await?;
        Ok(tx)
    }
}
