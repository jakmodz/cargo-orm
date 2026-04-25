use corrosion_orm_core::validation::Validation;
use syn::Ident;

/// Represents all validation rules for a single struct field
///
/// A single field can have multiple validations applied to it,
/// for example: `#[not_null] #[size(min=1, max=100)] #[email]`
pub(crate) struct ValidationRule {
    /// The field identifier
    pub ident: Ident,
    /// All validations applied to this field
    pub validation: Validation,
}

impl ValidationRule {
    /// Creates a new ValidationRule with a single validation
    ///
    /// # Arguments
    /// * `ident` - The field identifier
    /// * `validation` - A single validation to apply
    pub fn new(ident: Ident, validation: Validation) -> Self {
        Self { ident, validation }
    }
}
