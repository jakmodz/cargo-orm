use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    driver::{
        connection_config::ConnectionConfig,
        connection_pool::{ConnectionGuard, ConnectionPool},
        error::DriverError,
        sqlite_driver::sqlite_config::SqliteConfig,
    },
    error::CargoOrmError,
};
use std::time::Duration;
impl ConnectionPool for sqlx::SqlitePool {
    type Conn = sqlx::pool::PoolConnection<sqlx::Sqlite>;
    type Config = SqliteConfig;

    async fn new_pool(config: Self::Config) -> Result<Self, CargoOrmError> {
        config.validate()?;
        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections() as u32)
            .min_connections(config.min_connections() as u32)
            .idle_timeout(Duration::from_millis(config.connection_timeout_ms()))
            .connect(config.url())
            .await
            .map_err(DriverError::Sqlx)?;
        Ok(pool)
    }

    async fn acquire_conn(&self) -> Result<ConnectionGuard<Self>, CargoOrmError> {
        let conn = self.acquire().await.map_err(DriverError::Sqlx)?;
        Ok(ConnectionGuard::new(conn))
    }

    fn active_conn(&self) -> u32 {
        self.size() - self.num_idle() as u32
    }

    fn total_conn(&self) -> u32 {
        self.size()
    }

    async fn close(self) -> Result<(), CargoOrmError> {
        sqlx::SqlitePool::close(&self).await;
        Ok(())
    }
}
