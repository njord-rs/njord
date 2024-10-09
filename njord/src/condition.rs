//! BSD 3-Clause License
//!
//! Copyright (c) 2024, Marcus Cvjeticanin
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::query::QueryBuilder;

/// Represents a condition used in building SQL queries.
#[derive(Clone)]
pub enum Condition<'a> {
    /// Equality condition: column = value.
    Eq(String, Value<'a>),
    /// Inequality condition: column <> value.
    Ne(String, Value<'a>),
    /// Less than condition: column < value.
    Lt(String, Value<'a>),
    /// Greater than condition: column > value.
    Gt(String, Value<'a>),
    /// Less than or equal to condition: column <= value.
    Le(String, Value<'a>),
    /// Greater than or equal to condition: column >= value.
    Ge(String, Value<'a>),
    /// Logical AND condition.
    And(Box<Condition<'a>>, Box<Condition<'a>>),
    /// Logical OR condition.
    Or(Box<Condition<'a>>, Box<Condition<'a>>),
    /// In condition: column IN (value1, value2, ...).
    In(String, Vec<Value<'a>>),
    /// Not in condition: column NOT IN (value1, value2, ...).
    NotIn(String, Vec<Value<'a>>),
}

/// Required to implement support for subqueries and literals.
#[derive(Clone)]
pub enum Value<'a> {
    /// A literal value, such as a string or number.
    Literal(String),
    /// A subquery.
    Subquery(Box<dyn QueryBuilder<'a> + 'a>),
}

impl<'a> Value<'a> {
    pub fn is_numeric(&self) -> bool {
        match self {
            Value::Literal(literal) => Condition::is_numeric(literal),
            _ => false,
        }
    }
}

/// Implement Display for Value
impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(literal) => write!(f, "{}", literal),
            Value::Subquery(subquery) => write!(f, "{}", subquery.to_sql()),
        }
    }
}

impl<'a> Condition<'a> {
    /// Checks if the given value is numeric.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is numeric, otherwise `false`.
    fn is_numeric(value: &str) -> bool {
        value.parse::<f64>().is_ok() || value.parse::<i64>().is_ok()
    }

    /// Builds the SQL representation of the condition.
    ///
    /// # Returns
    ///
    /// Returns a `String` representing the SQL condition.
    pub fn build(&self) -> String {
        match self {
            Condition::Eq(column, value) => {
                // If contains a dot, assume it's a table.column
                if column.contains('.') {
                    format!("{} = {}", column, value)
                } else if value.is_numeric() {
                    format!("{} = {}", column, value)
                } else {
                    format!("{} = '{}'", column, value)
                }
            }
            Condition::Ne(column, value) => {
                if value.is_numeric() {
                    format!("{} <> {}", column, value)
                } else {
                    format!("{} <> '{}'", column, value)
                }
            }
            Condition::Lt(column, value) => {
                if value.is_numeric() {
                    format!("{} < {}", column, value)
                } else {
                    format!("{} < '{}'", column, value)
                }
            }
            Condition::Gt(column, value) => {
                if value.is_numeric() {
                    format!("{} > {}", column, value)
                } else {
                    format!("{} > '{}'", column, value)
                }
            }
            Condition::Le(column, value) => {
                if value.is_numeric() {
                    format!("{} <= {}", column, value)
                } else {
                    format!("{} <= '{}'", column, value)
                }
            }
            Condition::Ge(column, value) => {
                if value.is_numeric() {
                    format!("{} >= {}", column, value)
                } else {
                    format!("{} >= '{}'", column, value)
                }
            }
            Condition::And(left, right) => format!("({}) AND ({})", left.build(), right.build()),
            Condition::Or(left, right) => format!("({}) OR ({})", left.build(), right.build()),
            Condition::In(column, values) => {
                let values = values
                    .iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{} IN ({})", column, values)
            }
            Condition::NotIn(column, values) => {
                let values = values
                    .iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{} NOT IN ({})", column, values)
            }
        }
    }
}
