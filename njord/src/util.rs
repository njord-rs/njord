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

use std::sync::Arc;

use crate::condition::Condition;
use crate::table::Table;

/// Represents the type of SQL join.
#[derive(Clone, Debug)]
pub enum JoinType {
    /// Inner Join - Includes only matching rows from both tables.
    Inner,
    /// Left Join - Includes all rows from the left table and matching rows from the right table.
    Left,
    /// Right Join - Includes all rows from the right table and matching rows from the left table.
    Right,
    /// Full Join - Includes all rows from both tables.
    Full,
}

/// Represents a join operation in an SQL query.
#[derive(Clone)]
pub struct Join<'a> {
    /// The type of join (e.g., Inner, Left, Right, Full).
    pub join_type: JoinType,
    /// The table involved in the join.
    pub table: Arc<dyn Table>,
    /// The condition specifying how the tables are joined.
    pub on_condition: Condition<'a>,
}

impl<'a> Join<'a> {
    /// Creates a new `Join` instance representing an SQL join operation.
    ///
    /// # Arguments
    ///
    /// * `join_type` - The type of join to perform, such as `Inner`, `Left`, `Right`, or `Full`.
    /// * `table` - A reference to the table being joined, wrapped in an `Arc<dyn Table>` for shared ownership.
    /// * `on_condition` - A `Condition` specifying the criteria for joining the tables.
    ///
    /// # Returns
    ///
    /// A new `Join` instance representing the specified join operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use njord::condition::{Condition, Value};
    /// use njord::util::{Join, JoinType};
    /// use njord_derive::Table;
    ///
    /// #[derive(Table)]
    /// struct Categories {
    ///     id: i32,
    ///     name: String,
    /// }
    ///
    /// #[derive(Table)]
    /// struct Products {
    ///     id: i32,
    ///     category_id: i32,
    ///     name: String,
    ///     price: f64,
    /// }
    ///
    /// fn main() {
    ///     let categories = Categories {
    ///         id: 1,
    ///         name: "Electronics".to_string(),
    ///     };
    ///
    ///     let products = Products {
    ///         id: 101,
    ///         category_id: 1,
    ///         name: "Smartphone".to_string(),
    ///         price: 699.99,
    ///     };
    ///
    ///     let join_condition = Condition::Eq(
    ///         "categories.id".to_string(),
    ///         Value::Literal("products.category_id".to_string()),
    ///     );
    ///
    ///     let join = Join::new(
    ///         JoinType::Inner,
    ///         Arc::new(products),
    ///         join_condition,
    ///     );
    /// }
    /// ```
    pub fn new(join_type: JoinType, table: Arc<dyn Table>, on_condition: Condition<'a>) -> Self {
        Join {
            join_type,
            table,
            on_condition,
        }
    }
}

/// Converts values for SQL INSERT
///
/// # Arguments
///
/// * 'values' - A vector of strings to be parsed.
///
/// # Returns
///
/// A new vector of strings with surrounding single quotes if it contains text.
pub fn convert_insert_values(values: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for item in values {
        if let Ok(parsed_int) = item.parse::<i32>() {
            result.push(parsed_int.to_string());
        } else if let Ok(parsed_float) = item.parse::<f64>() {
            result.push(parsed_float.to_string());
        } else if item.eq_ignore_ascii_case("true") {
            result.push("true".to_string());
        } else if item.eq_ignore_ascii_case("false") {
            result.push("false".to_string());
        } else {
            // if it's not true or false, surround it with single quotes and push it.
            result.push(format!("'{}'", item));
        }
    }

    result
}
