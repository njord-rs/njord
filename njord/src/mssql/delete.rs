//! BSD 3-Clause License
//!
//! Copyright (c) 2024
//!     Marcus Cvjeticanin
//!     Chase Willden
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
    mssql::util::{generate_where_condition_str, remove_quotes_and_backslashes},
};

use log::info;

use crate::table::Table;

use super::Connection;

/// Constructs a new DELETE query builder.
///
/// # Arguments
///
/// * `conn` - A `Connection` to the MSSQL database.
///
/// # Returns
///
/// A `DeleteQueryBuilder` instance.
pub fn delete<T: Table + Default>(conn: &mut Connection) -> DeleteQueryBuilder<T> {
    DeleteQueryBuilder::new(conn)
}

/// A builder for constructing DELETE queries.
pub struct DeleteQueryBuilder<'a, T: Table + Default> {
    conn: &'a mut Connection,
    table: Option<T>,
    where_condition: Option<Condition<'a>>,
}

impl<'a, T: Table + Default> DeleteQueryBuilder<'a, T> {
    /// Creates a new `DeleteQueryBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `conn` - A `Connection` to the MSSQL database.
    pub fn new(conn: &'a mut Connection) -> Self {
        DeleteQueryBuilder {
            conn,
            table: None,
            where_condition: None,
        }
    }

    /// Sets the table from which to delete data.
    ///
    /// # Arguments
    ///
    /// * `table` - An instance of the table from which to delete data.
    pub fn from(mut self, table: T) -> Self {
        self.table = Some(table);
        self
    }

    /// Sets the WHERE clause condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to be applied in the WHERE clause.
    pub fn where_clause(mut self, condition: Condition<'a>) -> Self {
        self.where_condition = Some(condition);
        self
    }

    /// Builds and executes the DELETE query.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the deletion operation.
    pub async fn build(self) -> Result<(), String> {
        let table_name = self
            .table
            .as_ref()
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = remove_quotes_and_backslashes(&table_name);
        let where_condition_str = generate_where_condition_str(self.where_condition);

        // Construct the query based on defined variables above
        let query = format!("DELETE FROM {} {}", table_name_str, where_condition_str,);

        info!("{}", query);

        // Execute SQL
        match self.conn.client.execute(&query, &[]).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
