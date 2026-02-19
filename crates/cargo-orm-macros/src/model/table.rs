use deluxe::ExtractAttributes;

use crate::model::Field;

#[derive(ExtractAttributes)]
#[deluxe(attributes(table))]
pub struct TableAttribute{
    #[deluxe(default = String::from(""))]
    pub(crate) name:String
}

pub struct TableData{
    pub ident: syn::Ident,
    pub name: String,
    pub fields: Vec<Field>,
}