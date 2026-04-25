use crate::validation::error::ValidationError;

pub trait Validator<T> {
    fn validate(
        &self,
        field_name: &str,
        value: &T,
        msg: Option<&String>,
    ) -> Result<(), ValidationError>;
}

pub struct NotNullValidator;

impl Validator<String> for NotNullValidator {
    fn validate(
        &self,
        field_name: &str,
        value: &String,
        msg: Option<&String>,
    ) -> Result<(), ValidationError> {
        let err = msg.map_or(
            ValidationError::NullField {
                field: field_name.to_string(),
            },
            |msg| ValidationError::CustomError {
                msg: msg.to_string(),
            },
        );

        if value.is_empty() {
            return Err(err);
        }
        Ok(())
    }
}
impl<E> Validator<Option<E>> for NotNullValidator {
    fn validate(
        &self,
        field_name: &str,
        value: &Option<E>,
        msg: Option<&String>,
    ) -> Result<(), ValidationError> {
        let err = msg.map_or(
            ValidationError::NullField {
                field: field_name.to_string(),
            },
            |msg| ValidationError::CustomError {
                msg: msg.to_string(),
            },
        );

        if value.is_none() {
            return Err(err);
        }
        Ok(())
    }
}
pub struct SizeValidator {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl Validator<String> for SizeValidator {
    fn validate(
        &self,
        field_name: &str,
        value: &String,
        msg: Option<&String>,
    ) -> Result<(), ValidationError> {
        let err = msg.map_or(
            ValidationError::SizeError {
                field: field_name.to_string(),
            },
            |msg| ValidationError::CustomError {
                msg: msg.to_string(),
            },
        );

        if let Some(min) = self.min
            && value.len() < min
        {
            return Err(err);
        }

        if let Some(max) = self.max
            && value.len() > max
        {
            return Err(err);
        }
        Ok(())
    }
}

pub struct PatternValidator {
    pub pattern: regex::Regex,
}

impl PatternValidator {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: regex::Regex::new(pattern)?,
        })
    }
}

impl Validator<String> for PatternValidator {
    fn validate(
        &self,
        field_name: &str,
        value: &String,
        msg: Option<&String>,
    ) -> Result<(), ValidationError> {
        let err = msg.map_or(
            ValidationError::PatternError {
                field: field_name.to_string(),
                pattern: self.pattern.as_str().to_string(),
            },
            |msg| ValidationError::CustomError {
                msg: msg.to_string(),
            },
        );

        if !self.pattern.is_match(value) {
            return Err(err);
        }
        Ok(())
    }
}
