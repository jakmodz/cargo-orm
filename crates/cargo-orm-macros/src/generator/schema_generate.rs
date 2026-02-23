use proc_macro2::TokenStream;

use crate::TableData;
use quote::quote;

pub fn generate_schema_impl(table: &TableData)->TokenStream{
    let mut fields:Vec<TokenStream> = Vec::new();
    for field in table.fields.iter() {
        let field_name = &field.name;
        let field_type = &field.ty;
        //TODO: generate schema
        let is_primary_key = true;
        let is_nullable = field.is_nullable;
        fields.push(quote!{
            cargo_orm_core::schema::table::ColumnSchemaModel::new::<#field_type>(String::from(#field_name),#is_primary_key,#is_nullable,<#field_type>::default())
        });
    }
    
    let table_name = &table.name;
    let struct_ident = &table.ident; 
    quote!{
        impl cargo_orm_core::schema::table::TableSchema for #struct_ident{
            fn get_table_name()->&'static str{
                #table_name
            }
            fn get_schema()->cargo_orm_core::schema::table::TableSchemaModel{
                cargo_orm_core::schema::table::TableSchemaModel{
                    name: String::from(#table_name),
                    fields: vec!(#(#fields),*),
                    indexes: Vec::new()
                }
            }
        }
    }
}

