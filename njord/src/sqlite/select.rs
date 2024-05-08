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

use crate::{
    condition::Condition,
    sqlite::util::{
        generate_group_by_str, generate_having_str, generate_limit_str, generate_offset_str,
        generate_order_by_str, generate_where_condition_str, remove_quotes_and_backslashes,
    },
};
use std::collections::HashMap;

use rusqlite::{Connection, Result};

use log::info;
use rusqlite::types::Value;

use crate::table::Table;

/// Constructs a new SELECT query builder.
///
/// # Arguments
///
/// * `conn` - A `rusqlite::Connection` to the SQLite database.
/// * `columns` - A vector of strings representing the columns to be selected.
///
/// # Returns
///
/// A `SelectQueryBuilder` instance.
pub fn select<T: Table + Default>(conn: Connection, columns: Vec<String>) -> SelectQueryBuilder<T> {
    SelectQueryBuilder::new(conn, columns)
}

/// A builder for constructing SELECT queries.
pub struct SelectQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
    columns: Vec<String>,
    where_condition: Option<Condition>,
    distinct: bool,
    group_by: Option<Vec<String>>,
    order_by: Option<HashMap<Vec<String>, String>>,
    limit: Option<usize>,
    offset: Option<usize>,
    having_condition: Option<Condition>,
}

impl<T: Table + Default> SelectQueryBuilder<T> {
    /// Creates a new `SelectQueryBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `conn` - A `rusqlite::Connection` to the SQLite database.
    /// * `columns` - A vector of strings representing the columns to be selected.
    pub fn new(conn: Connection, columns: Vec<String>) -> Self {
        SelectQueryBuilder {
            conn,
            table: None,
            columns,
            where_condition: None,
            distinct: false,
            group_by: None,
            order_by: None,
            limit: None,
            offset: None,
            having_condition: None,
        }
    }

    /// Sets the columns to be selected.
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of strings representing the columns to be selected.
    pub fn select(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self
    }

    /// Sets the DISTINCT keyword for the query.
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Sets the table from which to select data.
    ///
    /// # Arguments
    ///
    /// * `table` - The table from which to select data.
    pub fn from(mut self, table: T) -> Self {
        self.table = Some(table);
        self
    }

    /// Sets the WHERE clause condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to be applied in the WHERE clause.
    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.where_condition = Some(condition);
        self
    }

    /// Sets the GROUP BY clause columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of strings representing the columns to be grouped by.
    pub fn group_by(mut self, columns: Vec<String>) -> Self {
        self.group_by = Some(columns);
        self
    }

    /// Sets the ORDER BY clause columns and order direction.
    ///
    /// # Arguments
    ///
    /// * `col_and_order` - A HashMap containing column names as keys and order direction as values.
    pub fn order_by(mut self, col_and_order: HashMap<Vec<String>, String>) -> Self {
        self.order_by = Some(col_and_order);
        self
    }

    /// Sets the LIMIT clause for the query.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of rows to limit the result to.
    pub fn limit(mut self, count: usize) -> Self {
        self.limit = Some(count);
        self
    }

    /// Sets the OFFSET clause for the query.
    ///
    /// # Arguments
    ///
    /// * `offset` - The number of rows to skip.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the HAVING clause condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to be applied in the HAVING clause.
    pub fn having(mut self, condition: Condition) -> Self {
        self.having_condition = Some(condition);
        self
    }

    /// Builds and executes the SELECT query.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of selected table rows if successful,
    /// or a `rusqlite::Error` if an error occurs during the execution.
    pub fn build(self) -> Result<Vec<T>> {
        let columns_str = self.columns.join(", ");

        let table_name = self
            .table
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = remove_quotes_and_backslashes(&table_name);
        let distinct_str = if self.distinct { "DISTINCT " } else { "" };
        let where_condition_str = generate_where_condition_str(self.where_condition);
        let group_by_str = generate_group_by_str(&self.group_by);
        let order_by_str = generate_order_by_str(&self.order_by);
        let limit_str = generate_limit_str(self.limit);
        let offset_str = generate_offset_str(self.offset);
        let having_str =
            generate_having_str(self.group_by.is_some(), self.having_condition.as_ref()); // Having should only be added if group_by is present

        // Construct the query based on defined variables above
        let query = format!(
            "SELECT {}{} FROM {} {} {} {} {} {}",
            distinct_str,
            columns_str,
            table_name_str,
            where_condition_str,
            group_by_str,
            having_str,
            order_by_str,
            format!("{} {}", limit_str, offset_str),
        );

        info!("{}", query);
        println!("{}", query);

        // Prepare SQL statement
        let mut stmt = self.conn.prepare(query.as_str())?;

        let iter = stmt.query_map((), |row| {
            // Dynamically create an instance of the struct based on the Table trait
            let mut instance = T::default();
            let columns = instance.get_column_fields();

            for (index, column) in columns.iter().enumerate() {
                // Use the index to get the value from the row and set it in the struct
                let value = row.get::<usize, Value>(index)?;

                let string_value = match value {
                    Value::Integer(val) => val.to_string(),
                    Value::Null => String::new(),
                    Value::Real(val) => val.to_string(),
                    Value::Text(val) => val.to_string(),
                    Value::Blob(val) => String::from_utf8_lossy(&val).to_string(),
                };

                instance.set_column_value(column, &string_value);
            }

            Ok(instance)
        })?;

        let result: Result<Vec<T>> = iter
            .map(|row_result| row_result.and_then(|row| Ok(row)))
            .collect::<Result<Vec<T>>>();

        result.map_err(|err| err.into())
    }
}
