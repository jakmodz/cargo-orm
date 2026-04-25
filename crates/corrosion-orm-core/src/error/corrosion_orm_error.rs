use crate::{
    driver::{connection_config::ConnectionConfigError, error::DriverError},
    schema::table::SchemaValidationError,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CorrosionOrmError {
    #[error(transparent)]
    SchemaValidationErrors(#[from] SchemaValidationError),
    #[error(transparent)]
    DriverError(#[from] DriverError),
    #[error(transparent)]
    ValidationError(#[from] std::io::Error),
}
impl From<ConnectionConfigError> for CorrosionOrmError {
    fn from(e: ConnectionConfigError) -> Self {
        CorrosionOrmError::DriverError(DriverError::InvalidConfiguration(e))
    }
}
