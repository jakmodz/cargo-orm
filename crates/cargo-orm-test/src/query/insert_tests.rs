#[cfg(test)]
mod tests {
    use cargo_orm_core::dialect::sqlite_dialect::sqlite::SqliteDialect;
    use cargo_orm_core::query::insert::Insert;
    use cargo_orm_core::query::query_type::{QueryContext, Value};
    use cargo_orm_core::query::to_sql::ToSql;
    use std::borrow::Cow;
    #[test]
    fn test_insert_sqlite() {
        let mut ctx = QueryContext::new();
        let insert = Insert::new("users")
            .columns(vec![Cow::Borrowed("name"), Cow::Borrowed("age")])
            .values(vec![Value::String("Alice".to_string()), Value::Int(30)]);
        insert.to_sql(&mut ctx, &SqliteDialect);
        assert_eq!(ctx.values.len(), 2);
        assert_eq!(ctx.values[0], Value::String("Alice".to_string()));
        assert_eq!(ctx.values[1], Value::Int(30));
        insta::assert_snapshot!(ctx.sql);
    }
}
