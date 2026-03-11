use crate::driver::{
    connection_pool::ConnectionPool, sql_driver::SqlDriver,
    sqlite_driver::sqlite_connection_pool::CargoSqlitePool,
};

pub struct SqliteDriver {
    pool: CargoSqlitePool,
}

impl SqlDriver for SqliteDriver {
    type Pool = CargoSqlitePool;

    async fn new(
        config: <Self::Pool as crate::driver::connection_pool::ConnectionPool>::Config,
    ) -> Result<Self, crate::error::CargoOrmError> {
        let pool = CargoSqlitePool::new_pool(config).await?;
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
