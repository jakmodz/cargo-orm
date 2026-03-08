use crate::error::CargoOrmError;

pub trait Connection: Sized + Sync + Send {
    async fn ping(&self) -> Result<(), CargoOrmError>;
    fn is_valid(&self) -> bool;
}
