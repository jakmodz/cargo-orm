use thiserror::Error;
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("The field {field} is null")]
    NullField { field: String },
    #[error("The field {field} is out of size range")]
    SizeError { field: String },
    #[error("The field {field} does not match the pattern {pattern}")]
    PatternError { field: String, pattern: String },
    #[error("{msg}")]
    CustomError { msg: String },
    #[error(transparent)]
    RegexError(#[from] regex::Error),
}
