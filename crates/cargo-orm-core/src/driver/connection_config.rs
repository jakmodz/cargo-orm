use thiserror::Error;

use crate::driver::error::DriverError;
use crate::error::CargoOrmError;
#[derive(Debug, Error)]
pub enum ConnectionConfigError {
    #[error("Url to database is empty")]
    EmptyUrl,
}

pub trait ConnectionConfig {
    fn url(&self) -> &str;
    fn max_connections(&self) -> usize {
        10
    }
    fn min_connections(&self) -> usize {
        1
    }
    fn connection_timeout_ms(&self) -> u64 {
        5000
    }
    fn validate(&self) -> Result<(), CargoOrmError> {
        if self.url().is_empty() {
            return Err(CargoOrmError::DriverError(
                DriverError::InvalidConfiguration(ConnectionConfigError::EmptyUrl),
            ));
        }
        Ok(())
    }
}
