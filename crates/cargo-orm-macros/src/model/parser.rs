use syn::{DeriveInput,Fields, spanned::Spanned};
use crate::model::{ColumnnAttribute, Field,TableAttribute, TableData};

pub fn parse_model(ast: &mut DeriveInput) -> syn::Result<TableData> {
    let table_attribute: TableAttribute = deluxe::extract_attributes(ast)?;
    #[allow(unused_assignments)]
    let mut fields: Vec<Field> = Vec::new();
    if let syn::Data::Struct(s) = &mut ast.data {
        fields = parse_fields(&mut s.fields)?;
    } else {
        return Err(syn::Error::new(ast.span(), "Model can only be derived for structs"));
    }
    
    syn::Result::Ok(TableData {
        ident: ast.ident.clone(),
        name: table_attribute.name.is_empty().then(|| ast.ident.to_string()).unwrap_or(table_attribute.name),
        fields
    })
}
fn parse_fields(fields:&mut Fields)->syn::Result<Vec<Field>>{ 
    let mut fields_vec = Vec::new();
    for field in fields.iter_mut() {
        let column_attr: ColumnnAttribute = deluxe::extract_attributes(field)?;
        let parsed_field = Field::from((column_attr, &*field));
        fields_vec.push(parsed_field);
    }
    syn::Result::Ok(fields_vec)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use syn::{parse_quote, Data, Fields};

    #[test]
    fn test_parse_model() {
        let input: DeriveInput = parse_quote! {
            #[table(name = "users")]
            struct User {
                #[Column(name = "id", primary_key)]
                id: i32,
                #[Column(name = "username", unique)]
                username: String,
                #[Column(name = "email", unique = false, nullable)]
                email: Option<String>,
            }
        };

        let table_data = parse_model(&mut input.clone()).unwrap();
        assert_eq!(table_data.name, "users");
        assert_eq!(table_data.fields.len(), 3);
        assert_eq!(table_data.fields[0].name, "id");
        assert!(table_data.fields[0].is_primary_key);
        assert_eq!(table_data.fields[1].name, "username");
        assert!(table_data.fields[1].is_unique);
        assert_eq!(table_data.fields[2].name, "email");
        assert_eq!(table_data.fields[2].is_unique,false);
        assert!(table_data.fields[2].is_nullable);
    }
}