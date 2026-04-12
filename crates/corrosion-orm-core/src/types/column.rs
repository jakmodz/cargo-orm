//! Type-safe column wrapper structs for compile-time type checking in queries.

use crate::{
    query::{
        query_type::Value,
        where_clause::{Condition, WhereClause, WhereClauseType},
    },
    types::ColumnTrait,
};

/// A string column wrapper with methods for string-specific operations.
pub struct StringColumn<C: ColumnTrait> {
    pub column: C,
}

impl<C: ColumnTrait> StringColumn<C> {
    pub const fn new(column: C) -> Self {
        Self { column }
    }
    /// Creates a `WhereClause` for equality comparison with a value.
    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::eq(self.column, val)
    }

    /// Creates a `WhereClause` for inequality comparison with a value.
    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::ne(self.column, val)
    }
    /// Creates a `WhereClause` for `LIKE` comparison with a value.
    pub fn like<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.column,
            val,
        )))
    }
    /// Creates a `WhereClause` for `NOT LIKE` comparison with a value.
    pub fn not_like<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::not_like(
            self.column,
            val,
        )))
    }
    /// Creates a `WhereClause` for `CONTAINS` comparison with a value.
    pub fn contains<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        let val_str = match val.into() {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.column,
            Value::String(format!("%{}%", val_str)),
        )))
    }
    /// Creates a `WhereClause` for `STARTS WITH` comparison with a value.
    pub fn starts_with<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        let val_str = match val.into() {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.column,
            Value::String(format!("{}%", val_str)),
        )))
    }
    /// Creates a `WhereClause` for `ENDS WITH` comparison with a value.
    pub fn ends_with<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        let val_str = match val.into() {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        WhereClause::new(WhereClauseType::Condition(Condition::like(
            self.column,
            Value::String(format!("%{}", val_str)),
        )))
    }
    /// Creates a `WhereClause` for `IN` comparison with a list of values.
    pub fn in_<V: Into<Value>>(self, vals: Vec<V>) -> WhereClause<C> {
        let values: Vec<Value> = vals.into_iter().map(|v| v.into()).collect();
        WhereClause::in_(self.column, values)
    }
    /// Creates a `WhereClause` for `IS NULL` comparison.
    pub fn is_null(self) -> WhereClause<C> {
        WhereClause::is_null(self.column)
    }
}

/// A numeric column wrapper with methods for numeric-specific operations.
pub struct NumericColumn<C: ColumnTrait> {
    pub column: C,
}

impl<C: ColumnTrait> NumericColumn<C> {
    pub const fn new(column: C) -> Self {
        Self { column }
    }
    /// Creates a `WhereClause` for equality comparison with a value.
    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::eq(self.column, val)
    }
    /// Creates a `WhereClause` for inequality comparison with a value.
    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::ne(self.column, val)
    }
    /// Creates a `WhereClause` for greater-than comparison with a value.
    pub fn gt<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::gt(self.column, val)
    }
    /// Creates a `WhereClause` for greater-than-or-equal comparison with a value.
    pub fn gte<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::gte(self.column, val)))
    }
    /// Creates a `WhereClause` for less-than comparison with a value.
    pub fn lt<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::lt(self.column, val)
    }
    /// Creates a `WhereClause` for less-than-or-equal comparison with a value.
    pub fn lte<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::lte(self.column, val)))
    }

    pub fn between<V: Into<Value>>(self, min: V, max: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::between(
            self.column,
            min,
            max,
        )))
    }

    pub fn in_<V: Into<Value>>(self, vals: Vec<V>) -> WhereClause<C> {
        let values: Vec<Value> = vals.into_iter().map(|v| v.into()).collect();
        WhereClause::in_(self.column, values)
    }

    pub fn is_null(self) -> WhereClause<C> {
        WhereClause::is_null(self.column)
    }
}

/// A date/timestamp column wrapper with methods for date-specific operations.
pub struct DateLikeColumn<C: ColumnTrait> {
    column: C,
}

impl<C: ColumnTrait> DateLikeColumn<C> {
    pub const fn new(column: C) -> Self {
        Self { column }
    }

    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::eq(self.column, val)
    }

    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::ne(self.column, val)
    }

    pub fn gt<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::gt(self.column, val)
    }

    pub fn gte<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::gte(self.column, val)))
    }

    pub fn lt<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::lt(self.column, val)
    }

    pub fn lte<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::lte(self.column, val)))
    }

    pub fn between<V: Into<Value>>(self, min: V, max: V) -> WhereClause<C> {
        WhereClause::new(WhereClauseType::Condition(Condition::between(
            self.column,
            min,
            max,
        )))
    }

    pub fn is_null(self) -> WhereClause<C> {
        WhereClause::is_null(self.column)
    }
}

/// A boolean column wrapper with methods for boolean-specific operations.
pub struct BooleanColumn<C: ColumnTrait> {
    column: C,
}

impl<C: ColumnTrait> BooleanColumn<C> {
    pub const fn new(column: C) -> Self {
        Self { column }
    }

    pub fn eq<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::eq(self.column, val)
    }

    pub fn ne<V: Into<Value>>(self, val: V) -> WhereClause<C> {
        WhereClause::ne(self.column, val)
    }

    pub fn is_null(self) -> WhereClause<C> {
        WhereClause::is_null(self.column)
    }
}
