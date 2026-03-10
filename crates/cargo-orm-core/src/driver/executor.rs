use crate::error::CargoOrmError;

pub trait Executor: Sized + Send + Sync {
    async fn execute_query(&mut self, sql: &str) -> Result<u64, CargoOrmError>;
}
