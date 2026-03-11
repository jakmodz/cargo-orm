use crate::{
    driver::{
        connection::Connection, connection_pool::ConnectionPool, error::DriverError,
        executor::Executor,
    },
    error::CargoOrmError,
};

pub struct Transaction<P: ConnectionPool> {
    conn: Option<P::Conn>,
}

impl<P: ConnectionPool> Transaction<P> {
    /// Creates a new transaction with the given connection.
    pub fn new(conn: P::Conn) -> Self {
        Self { conn: Some(conn) }
    }

    /// Commits the transaction.
    pub async fn commit(mut self) -> Result<(), CargoOrmError> {
        if let Some(mut conn) = self.conn.take() {
            conn.commit_transaction().await?;
            Ok(())
        } else {
            Err(CargoOrmError::DriverError(DriverError::ConnectionClosed))
        }
    }
    /// Rolls back the transaction.
    pub async fn rollback(mut self) -> Result<(), CargoOrmError> {
        if let Some(mut conn) = self.conn.take() {
            conn.rollback_transaction().await?;
            Ok(())
        } else {
            Err(CargoOrmError::DriverError(DriverError::ConnectionClosed))
        }
    }
}

impl<P: ConnectionPool> Drop for Transaction<P> {
    fn drop(&mut self) {
        if let Some(mut conn) = self.conn.take() {
            conn.rollback_blocking();
        }
    }
}

impl<P: ConnectionPool> Executor for Transaction<P> {
    async fn execute_query(&mut self, sql: &str) -> Result<u64, CargoOrmError> {
        match self.conn.as_mut() {
            Some(conn) => conn.execute_query(sql).await,
            None => Err(CargoOrmError::DriverError(DriverError::ConnectionClosed)),
        }
    }
}
