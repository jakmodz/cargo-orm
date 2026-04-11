//! Type-safe column wrapper structs for compile-time type checking in queries.

use crate::query::{
    query_type::Value,
    where_clause::{Condition, WhereClause, WhereClauseType},
};

/// A string column wrapper with methods for string-specific operations.
pub struct StringColumn {
    pub name: &'static str,
}

impl StringColumn {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
    /// Creates a `WhereClause` for equality comparison with a value.
    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::eq(self.name, val)
    }

    /// Creates a `WhereClause` for inequality comparison with a value.
    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::ne(self.name, val)
    }
    /// Creates a `WhereClause` for `LIKE` comparison with a value.
    pub fn like<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::like(self.name, val)))
    }
    /// Creates a `WhereClause` for `NOT LIKE` comparison with a value.
    pub fn not_like<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::not_like(
            self.name, val,
        )))
    }
    /// Creates a `WhereClause` for `CONTAINS` comparison with a value.
    pub fn contains<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        let val_str = match val.into() {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.name,
            Value::String(format!("%{}%", val_str)),
        )))
    }
    /// Creates a `WhereClause` for `STARTS WITH` comparison with a value.
    pub fn starts_with<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        let val_str = match val.into() {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.name,
            Value::String(format!("{}%", val_str)),
        )))
    }
    /// Creates a `WhereClause` for `ENDS WITH` comparison with a value.
    pub fn ends_with<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        let val_str = match val.into() {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.name,
            Value::String(format!("%{}", val_str)),
        )))
    }
    /// Creates a `WhereClause` for `IN` comparison with a list of values.
    pub fn in_<V: Into<Value>>(self, vals: Vec<V>) -> WhereClause<'static> {
        let values: Vec<Value> = vals.into_iter().map(|v| v.into()).collect();
        WhereClause::in_(self.name, values)
    }
    /// Creates a `WhereClause` for `IS NULL` comparison.
    pub fn is_null(self) -> WhereClause<'static> {
        WhereClause::is_null(self.name)
    }
}

/// A numeric column wrapper with methods for numeric-specific operations.
pub struct NumericColumn {
    pub name: &'static str,
}

impl NumericColumn {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
    /// Creates a `WhereClause` for equality comparison with a value.
    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::eq(self.name, val)
    }
    /// Creates a `WhereClause` for inequality comparison with a value.
    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::ne(self.name, val)
    }
    /// Creates a `WhereClause` for greater-than comparison with a value.
    pub fn gt<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::gt(self.name, val)
    }
    /// Creates a `WhereClause` for greater-than-or-equal comparison with a value.
    pub fn gte<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::gte(self.name, val)))
    }
    /// Creates a `WhereClause` for less-than comparison with a value.
    pub fn lt<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::lt(self.name, val)
    }
    /// Creates a `WhereClause` for less-than-or-equal comparison with a value.
    pub fn lte<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::lte(self.name, val)))
    }

    pub fn between<V: Into<Value>>(self, min: V, max: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::between(
            self.name, min, max,
        )))
    }

    pub fn in_<V: Into<Value>>(self, vals: Vec<V>) -> WhereClause<'static> {
        let values: Vec<Value> = vals.into_iter().map(|v| v.into()).collect();
        WhereClause::in_(self.name, values)
    }

    pub fn is_null(self) -> WhereClause<'static> {
        WhereClause::is_null(self.name)
    }
}

/// A date/timestamp column wrapper with methods for date-specific operations.
pub struct DateLikeColumn {
    pub name: &'static str,
}

impl DateLikeColumn {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::eq(self.name, val)
    }

    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::ne(self.name, val)
    }

    pub fn gt<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::gt(self.name, val)
    }

    pub fn gte<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::gte(self.name, val)))
    }

    pub fn lt<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::lt(self.name, val)
    }

    pub fn lte<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::lte(self.name, val)))
    }

    pub fn between<V: Into<Value>>(self, min: V, max: V) -> WhereClause<'static> {
        WhereClause::new(WhereClauseType::Condition(Condition::between(
            self.name, min, max,
        )))
    }

    pub fn is_null(self) -> WhereClause<'static> {
        WhereClause::is_null(self.name)
    }
}

/// A boolean column wrapper with methods for boolean-specific operations.
pub struct BooleanColumn {
    pub name: &'static str,
}

impl BooleanColumn {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::eq(self.name, val)
    }

    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<'static> {
        WhereClause::ne(self.name, val)
    }

    pub fn is_null(self) -> WhereClause<'static> {
        WhereClause::is_null(self.name)
    }
}
