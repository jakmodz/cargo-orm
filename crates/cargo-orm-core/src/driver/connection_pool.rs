use std::ops::{Deref, DerefMut};

use crate::error::CargoOrmError;

pub struct ConnectionGuard<'pool, P: ConnectionPool> {
    conn: Option<P::Conn>,
    pool: &'pool P,
}

impl<'pool, P: ConnectionPool> ConnectionGuard<'pool, P> {
    pub(crate) fn new(conn: P::Conn, pool: &'pool P) -> Self {
        Self {
            conn: Some(conn),
            pool,
        }
    }
}

impl<P: ConnectionPool> Deref for ConnectionGuard<'_, P> {
    type Target = P::Conn;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().expect("connection already released")
    }
}

impl<P: ConnectionPool> DerefMut for ConnectionGuard<'_, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().expect("connection already released")
    }
}
impl<P: ConnectionPool> Drop for ConnectionGuard<'_, P> {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let _ = self.pool.release_conn_sync(conn);
        }
    }
}

pub trait ConnectionPool: Sized + Send + Sync {
    type Conn: super::connection::Connection;
    type Config: super::connection_config::ConnectionConfig;

    async fn new(config: Self::Config) -> Result<Self, CargoOrmError>;
    async fn acquire_conn(&self) -> Result<ConnectionGuard<'_, Self>, CargoOrmError>;
    // async fn release_conn(&self, conn: Self::Conn) -> Result<(), CargoOrmError>;

    fn active_conn(&self) -> u32;
    fn total_conn(&self) -> u32;

    fn release_conn_sync(&self, conn: Self::Conn);
    async fn close(self) -> Result<(), CargoOrmError>;
}
