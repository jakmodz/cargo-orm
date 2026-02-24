use crate::types::{
    colum_type::{SqlType, ToSqlType},
    generation_strategy::GenerationType,
};
pub trait TableSchema {
    fn get_table_name() -> &'static str;
    fn get_schema() -> TableSchemaModel;
}

pub struct TableSchemaModel {
    pub name: String,
    pub fields: Vec<ColumnSchemaModel>,
    pub indexes: Vec<IndexModel>,
    pub primary_key: PrimaryKeyModel,
}

pub struct ColumnSchemaModel {
    pub name: String,
    pub is_nullable: bool,
    pub sql_type: SqlType,
}
pub struct IndexModel {
    pub name: String,
    pub fields: Vec<String>,
    pub unique: bool,
}
pub struct PrimaryKeyModel {
    pub name: String,
    pub generation_type: Option<GenerationType>,
    pub ty: SqlType,
}
impl ColumnSchemaModel {
    pub fn new<T: ToSqlType>(name: String, is_nullable: bool, filed_type: T) -> Self {
        Self {
            name,
            is_nullable,
            sql_type: filed_type.to_sql_type(),
        }
    }
}
impl PrimaryKeyModel {
    pub fn new<T: ToSqlType>(name: String, generation_type: Option<GenerationType>, ty: T) -> Self {
        Self {
            name,
            generation_type,
            ty: ty.to_sql_type(),
        }
    }
}
