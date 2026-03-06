use thiserror::Error;
use crate::schema::table::SchemaValidationError;

#[derive(Error,Debug)]
pub enum CargoOrmError {
    #[error(transparent)]
    SchemaValidationErrors(#[from] SchemaValidationError),
}