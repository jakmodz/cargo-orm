use crate::TableData;
use quote::quote;

pub(crate) fn generate_repository(table: &TableData) -> proc_macro2::TokenStream {
    let ident = &table.ident;
    let primary_key_ty = &table.primary_key.ty;

    let mut col_names = vec![table.primary_key.name.as_str()];
    for field in &table.fields {
        col_names.push(field.name.as_str());
    }

    let pk_ident = &table.primary_key.iden;
    let pk_name = &table.primary_key.name;
    let field_idents: Vec<_> = table.fields.iter().map(|f| &f.iden).collect();
    let field_names: Vec<_> = table.fields.iter().map(|f| &f.name).collect();

    quote! {
        impl<Db: cargo_orm_core::driver::executor::Executor> cargo_orm_core::model::repository::Repo<Db> for #ident {
            type PrimaryKey = #primary_key_ty;

            async fn save(&self, db: &mut Db) -> Result<Self, cargo_orm_core::error::CargoOrmError> {
                use cargo_orm_core::query::to_sql::ToSql;
                use cargo_orm_core::schema::table::TableSchema;
                use std::default::Default;

                let schema = Self::get_schema();
                let dialect = db.get_dialect();

                let is_new = self.#pk_ident == <#primary_key_ty as Default>::default();

                if is_new {
                    let mut insert_query = cargo_orm_core::query::insert::Insert::new(#ident::get_table_name())
                        .columns(vec![#(std::borrow::Cow::Borrowed(#col_names)),*]);

                    let mut values = Vec::new();
                    values.push(cargo_orm_core::query::query_type::Value::from(self.#pk_ident.clone()));

                    #(
                        values.push(cargo_orm_core::query::query_type::Value::from(self.#field_idents.clone()));
                    )*

                    insert_query = insert_query.values(values);
                    let mut ctx = cargo_orm_core::query::query_type::QueryContext::new();
                    insert_query.to_sql(&mut ctx, dialect);
                    db.execute_query(&mut ctx).await?;
                } else {
                    //TODO: update query instead of insert
                    todo!()
                }

                Ok(self.clone())
            }

            async fn get_all(db: &mut Db) -> Result<Vec<Self>, cargo_orm_core::error::CargoOrmError> {
                todo!()
            }
            async fn get_by_id(id: Self::PrimaryKey, db: &mut Db) -> Result<Self, cargo_orm_core::error::CargoOrmError> {
                todo!()
            }
            async fn delete_by_id(id: Self::PrimaryKey, db: &mut Db) -> Result<(), cargo_orm_core::error::CargoOrmError> {
                todo!()
            }
        }
    }
}
