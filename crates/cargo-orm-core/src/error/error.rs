use crate::{driver::error::DriverError, schema::table::SchemaValidationError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CargoOrmError {
    #[error(transparent)]
    SchemaValidationErrors(#[from] SchemaValidationError),
    #[error(transparent)]
    DriverError(#[from] DriverError),
}
