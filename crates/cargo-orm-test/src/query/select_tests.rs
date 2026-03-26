#[cfg(test)]
mod tests {
    use crate::User;
    use cargo_orm_core::dialect::sqlite_dialect::sqlite::SqliteDialect;
    use cargo_orm_core::query::query_type::QueryContext;
    use cargo_orm_core::query::select::Select;
    use cargo_orm_core::query::to_sql::ToSql;
    use cargo_orm_core::schema::table::TableSchema;
    #[test]
    fn test_select_create_from_table_schema_sqlite() {
        let mut ctx = QueryContext::new();
        let schema = User::get_schema();
        let select = Select::from(&schema);
        select.to_sql(&mut ctx, &SqliteDialect);
        assert_eq!(select.get_columns().len(), 2);
        insta::assert_snapshot!(ctx.sql);
    }
}
