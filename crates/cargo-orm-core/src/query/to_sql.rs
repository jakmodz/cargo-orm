use crate::{dialect::sql_dialect::SqlDialect, query::query_type::QueryContext};

pub trait ToSql {
    fn to_sql(&self, ctx: &mut QueryContext, dialect: &dyn SqlDialect);
}
