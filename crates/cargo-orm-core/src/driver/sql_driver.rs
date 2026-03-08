use crate::{driver::connection_pool::ConnectionPool, error::CargoOrmError};

pub trait SqlDriver: Sized + Sync + Send {
    type Pool: ConnectionPool;

    async fn new(config: <Self::Pool as ConnectionPool>::Config) -> Result<Self, CargoOrmError>;

    async fn execute(&self, sql: &str) -> Result<(), CargoOrmError>;
    async fn ping(&self) -> Result<(), CargoOrmError>;
    fn active_connections(&self) -> u32 {
        self.pool().active_conn()
    }
    fn pool(&self) -> &Self::Pool;
}
