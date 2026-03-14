#[allow(clippy::disallowed_types)]
use sqlx::{Connection, Executor};

use crate::{driver::error::DriverError, error::CargoOrmError};
pub struct CargoSqliteConnection {
    pub(crate) inner: sqlx::pool::PoolConnection<sqlx::Sqlite>,
}

impl crate::driver::connection::Connection for CargoSqliteConnection {
    async fn ping_conn(&mut self) -> Result<(), CargoOrmError> {
        self.inner.ping().await.map_err(DriverError::Sqlx)?;
        Ok(())
    }

    async fn execute_query(&mut self, query: &str) -> Result<u64, crate::error::CargoOrmError> {
        let count = self
            .inner
            .execute(query)
            .await
            .map_err(DriverError::Sqlx)?
            .rows_affected();
        Ok(count)
    }

    async fn is_valid(&mut self) -> bool {
        self.inner.ping().await.is_ok()
    }

    async fn begin_transaction(&mut self) -> Result<(), CargoOrmError> {
        sqlx::query("BEGIN IMMEDIATE")
            .execute(self.inner.as_mut())
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(())
    }

    async fn commit_transaction(&mut self) -> Result<(), CargoOrmError> {
        sqlx::query("COMMIT")
            .execute(self.inner.as_mut())
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(())
    }

    async fn rollback_transaction(&mut self) -> Result<(), CargoOrmError> {
        sqlx::query("ROLLBACK")
            .execute(self.inner.as_mut())
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(())
    }
}
