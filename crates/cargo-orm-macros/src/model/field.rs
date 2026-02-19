use deluxe::ExtractAttributes;
use syn::{Ident, Type};

#[derive(ExtractAttributes)]
#[deluxe(attributes(Column))]
pub struct ColumnnAttribute{
    #[deluxe(default = String::from(""))]
    pub(crate) name: String,
    #[deluxe(default = false)]
    pub(crate) unique: bool,
    #[deluxe(default = false)]
    pub(crate) nullable: bool,
    #[deluxe(default = false)]
    pub(crate) primary_key: bool,   
}

pub struct Field{
    #[allow(dead_code)]
    pub iden: Ident,
    pub name: String,
    pub ty: Type,
    pub is_primary_key:bool,
    #[allow(dead_code)]
    pub is_unique:bool,
    pub is_nullable:bool,
}
impl From<(ColumnnAttribute, &syn::Field)> for Field {
    fn from((attr, syn_field): (ColumnnAttribute, &syn::Field)) -> Self {
        let field_name = if attr.name.is_empty() {
            syn_field.ident.as_ref().unwrap().to_string()
        } else {
            attr.name
        };
        
        Field {
            iden: syn_field.ident.clone().unwrap(),
            name: field_name,
            ty: syn_field.ty.clone(),
            is_primary_key: attr.primary_key,
            is_unique: attr.unique,
            is_nullable: attr.nullable,
        }
    }
}
