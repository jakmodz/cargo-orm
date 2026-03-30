use crate::dialect::sql_dialect::SqlDialect;
use crate::schema::table::TableSchemaModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Int(i32),
    Int64(i64),
    Bool(bool),
}

macro_rules! impl_from_value {
    ($variant:ident, $ty:ty) => {
        impl From<$ty> for Value {
            fn from(v: $ty) -> Self {
                Value::$variant(v as _)
            }
        }
    };
}

impl_from_value!(Int, i32);
impl_from_value!(Int, i8);
impl_from_value!(Int, i16);
impl_from_value!(Int, u8);
impl_from_value!(Int, u16);
impl_from_value!(Int64, i64);
impl_from_value!(Int64, u32);
impl_from_value!(Bool, bool);

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl<T> From<Option<T>> for Value
where
    Value: From<T>,
    T: Default,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(inner) => Value::from(inner),
            None => Value::from(T::default()),
        }
    }
}

pub struct QueryContext {
    pub sql: String,
    pub values: Vec<Value>,
    pub(crate) placeholder_count: usize,
}

impl QueryContext {
    pub fn new() -> Self {
        Self {
            sql: String::new(),
            values: Vec::new(),
            placeholder_count: 0,
        }
    }
    pub fn push_bind_param(&mut self, value: Value, dialect: &dyn SqlDialect) {
        self.placeholder_count += 1;
        self.sql
            .push_str(&dialect.bind_param(&self.placeholder_count));
        self.values.push(value);
    }
    pub fn from_model(model: TableSchemaModel, dialect: &dyn SqlDialect) -> Self {
        let mut ctx = Self::new();
        ctx.sql = dialect.generate_ddl(&model).unwrap();
        ctx
    }
}
impl Default for QueryContext {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for QueryContext {
    fn from(sql: String) -> Self {
        Self {
            sql,
            values: Vec::new(),
            placeholder_count: 0,
        }
    }
}
impl From<&str> for QueryContext {
    fn from(sql: &str) -> Self {
        Self {
            sql: sql.to_string(),
            values: Vec::new(),
            placeholder_count: 0,
        }
    }
}
