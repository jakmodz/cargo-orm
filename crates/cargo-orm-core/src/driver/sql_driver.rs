use crate::{
    driver::{
        connection_pool::{ConnectionGuard, ConnectionPool},
        transaction::Transaction,
    },
    error::CargoOrmError,
};
/// Represents a SQL driver that provides connection pooling and transaction management.
pub trait SqlDriver: Sized + Sync + Send {
    type Pool: ConnectionPool;

    async fn new(config: <Self::Pool as ConnectionPool>::Config) -> Result<Self, CargoOrmError>;
    async fn acquire_conn(&self) -> Result<ConnectionGuard<Self::Pool>, CargoOrmError> {
        self.pool().acquire_conn().await
    }

    fn active_connections(&self) -> u32 {
        self.pool().active_conn()
    }

    fn pool(&self) -> &Self::Pool;
    async fn transaction(&self) -> Result<Transaction<Self::Pool>, CargoOrmError> {
        self.pool().begin_transaction().await
    }
}
