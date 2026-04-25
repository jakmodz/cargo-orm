use corrosion_orm_core::validation::{Validation, ValidationType};
use syn::{DataStruct, DeriveInput, Field, spanned::Spanned};

use crate::validation_parser::{
    validation_attributes::{EmailAttribute, NotNullAttribute, PatternAttribute, SizeAttribute},
    validation_field::ValidationRule,
};

pub(crate) fn parse_validation(ast: &mut DeriveInput) -> syn::Result<Vec<ValidationRule>> {
    if let syn::Data::Struct(data) = &mut ast.data {
        return Ok(parse_fields(data));
    }
    Err(syn::Error::new(
        ast.span(),
        "Validation can only be derived on structs",
    ))
}

fn parse_fields(struct_: &mut DataStruct) -> Vec<ValidationRule> {
    struct_
        .fields
        .iter_mut()
        .flat_map(parse_field_validations)
        .collect()
}

fn parse_field_validations(f: &mut Field) -> Vec<ValidationRule> {
    let mut rules = Vec::new();
    let ident = f
        .ident
        .as_ref()
        .expect("Field must have an identifier")
        .clone();

    if f.attrs.iter().any(|a| a.path().is_ident("NotNull"))
        && let Ok(a) = deluxe::extract_attributes::<_, NotNullAttribute>(&mut f.attrs)
    {
        rules.push(ValidationRule::new(
            ident.clone(),
            Validation::new(ValidationType::NotNull, a.message.unwrap_or_default()),
        ));
    }

    if f.attrs.iter().any(|a| a.path().is_ident("Size"))
        && let Ok(a) = deluxe::extract_attributes::<_, SizeAttribute>(&mut f.attrs)
    {
        rules.push(ValidationRule::new(
            ident.clone(),
            Validation::new(
                ValidationType::Size {
                    min: a.min,
                    max: a.max,
                },
                a.message.unwrap_or_default(),
            ),
        ));
    }

    if f.attrs.iter().any(|a| a.path().is_ident("Pattern"))
        && let Ok(a) = deluxe::extract_attributes::<_, PatternAttribute>(&mut f.attrs)
    {
        rules.push(ValidationRule::new(
            ident.clone(),
            Validation::new(
                ValidationType::Regex { pattern: a.pattern },
                a.message.unwrap_or_default(),
            ),
        ));
    }

    if f.attrs.iter().any(|a| a.path().is_ident("Email"))
        && let Ok(a) = deluxe::extract_attributes::<_, EmailAttribute>(&mut f.attrs)
    {
        rules.push(ValidationRule::new(
            ident.clone(),
            Validation::new(ValidationType::Email, a.message.unwrap_or_default()),
        ));
    }

    rules
}
