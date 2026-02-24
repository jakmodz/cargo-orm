#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlType {
    Integer,
    Varchar(usize),
    Text,
    Boolean,
    Date,
    Timestamp,
}

pub trait ToSqlType {
    fn to_sql_type(&self) -> SqlType;
}

impl ToSqlType for String {
    fn to_sql_type(&self) -> SqlType {
        if self.len() > 255 {
            return SqlType::Text;
        }
        SqlType::Varchar(255)
    }
}
impl ToSqlType for i32 {
    fn to_sql_type(&self) -> SqlType {
        SqlType::Integer
    }
}
impl ToSqlType for bool {
    fn to_sql_type(&self) -> SqlType {
        SqlType::Boolean
    }
}
impl ToSqlType for chrono::NaiveDate {
    fn to_sql_type(&self) -> SqlType {
        SqlType::Date
    }
}
impl ToSqlType for chrono::NaiveDateTime {
    fn to_sql_type(&self) -> SqlType {
        SqlType::Timestamp
    }
}

impl<T: ToSqlType + Default> ToSqlType for Option<T> {
    fn to_sql_type(&self) -> SqlType {
        T::default().to_sql_type()
    }
}
