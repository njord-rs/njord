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
