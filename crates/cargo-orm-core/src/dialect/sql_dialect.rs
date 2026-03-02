use crate::{schema::table::TableSchemaModel, types::column_type::SqlType};

pub trait SqlDialect {
    fn cast_type(&self,sql_type: &SqlType) -> String;
    /// Generates the DDL for the given schema.
    /// * It is default implementation for dialect *
    fn generate_ddl(&self, schema: &TableSchemaModel) -> String {
        let mut ddl = format!("CREATE TABLE {} (\n", schema.name);
        ddl.push_str(&format!("    {} {}\n", &schema.primary_key.name, 
            self.cast_type(&schema.primary_key.ty)));
        for field in schema.fields.iter() {
            ddl.push_str(&format!("    {} {}{}\n", field.name, 
                self.cast_type(&field.sql_type), 
                if field.is_nullable { "" } else { " NOT NULL" })
            );
        }
        ddl.push_str(");\n");
        ddl
    }
}

mod tests{
    
}