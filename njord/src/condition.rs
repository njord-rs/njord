/// Represents a condition used in building SQL queries.
pub enum Condition {
    /// Equality condition: column = value.
    Eq(String, String),
    /// Inequality condition: column <> value.
    Ne(String, String),
    /// Less than condition: column < value.
    Lt(String, String),
    /// Greater than condition: column > value.
    Gt(String, String),
    /// Less than or equal to condition: column <= value.
    Le(String, String),
    /// Greater than or equal to condition: column >= value.
    Ge(String, String),
    /// Logical AND condition.
    And(Box<Condition>, Box<Condition>),
    /// Logical OR condition.
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
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
                if Condition::is_numeric(value) {
                    format!("{} = {}", column, value)
                } else {
                    format!("{} = '{}'", column, value)
                }
            }
            Condition::Ne(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} <> {}", column, value)
                } else {
                    format!("{} <> '{}'", column, value)
                }
            }
            Condition::Lt(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} < {}", column, value)
                } else {
                    format!("{} < '{}'", column, value)
                }
            }
            Condition::Gt(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} > {}", column, value)
                } else {
                    format!("{} > '{}'", column, value)
                }
            }
            Condition::Le(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} <= {}", column, value)
                } else {
                    format!("{} <= '{}'", column, value)
                }
            }
            Condition::Ge(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} >= {}", column, value)
                } else {
                    format!("{} >= '{}'", column, value)
                }
            }
            Condition::And(left, right) => format!("({}) AND ({})", left.build(), right.build()),
            Condition::Or(left, right) => format!("({}) OR ({})", left.build(), right.build()),
        }
    }
}
