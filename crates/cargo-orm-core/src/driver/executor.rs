use crate::{
    dialect::sql_dialect::SqlDialect, error::CargoOrmError, query::query_type::QueryContext,
};

/// Executes SQL queries with a database entity.
#[trait_variant::make(Executor: Send)]
pub trait LocalExecutor: Sized + Send + Sync {
    async fn execute_query(&mut self, ctx: &mut QueryContext) -> Result<u64, CargoOrmError>;
    fn get_dialect(&self) -> &dyn SqlDialect;
}
