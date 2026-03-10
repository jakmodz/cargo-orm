use crate::error::CargoOrmError;

pub trait Connection: Sized + Sync + Send {
    async fn ping_conn(&mut self) -> Result<(), CargoOrmError>;
    async fn execute_query(&mut self, sql: &str) -> Result<u64, CargoOrmError>;
    //TODO query type
    async fn begin_transaction(&mut self) -> Result<(), CargoOrmError>;
    async fn commit_transaction(&mut self) -> Result<(), CargoOrmError>;
    async fn rollback_transaction(&mut self) -> Result<(), CargoOrmError>;
    async fn is_valid(&mut self) -> bool;

    fn rollback_blocking(&mut self) {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let _ = self.rollback_transaction().await;
            });
        });
    }
}
