use sqlx::{Connection, Executor, pool::PoolConnection};

use crate::{driver::error::DriverError, error::CargoOrmError};

impl crate::driver::connection::Connection for PoolConnection<sqlx::Sqlite> {
    async fn ping_conn(&mut self) -> Result<(), CargoOrmError> {
        self.ping().await.map_err(DriverError::Sqlx)?;
        Ok(())
    }

    async fn execute_query(&mut self, query: &str) -> Result<u64, crate::error::CargoOrmError> {
        let count = self
            .execute(query)
            .await
            .map_err(DriverError::Sqlx)?
            .rows_affected();
        Ok(count)
    }

    async fn is_valid(&mut self) -> bool {
        self.ping().await.is_ok()
    }

    async fn begin_transaction(&mut self) -> Result<(), CargoOrmError> {
        sqlx::query("BEGIN IMMEDIATE")
            .execute(&mut **self)
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(())
    }

    async fn commit_transaction(&mut self) -> Result<(), CargoOrmError> {
        sqlx::query("COMMIT")
            .execute(self.as_mut())
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(())
    }

    async fn rollback_transaction(&mut self) -> Result<(), CargoOrmError> {
        sqlx::query("ROLLBACK")
            .execute(self.as_mut())
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(())
    }
}
