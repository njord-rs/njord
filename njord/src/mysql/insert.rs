//! BSD 3-Clause License
//!
//! Copyright (c) 2024,
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

use crate::{query::QueryBuilder, table::Table};

use mysql::{prelude::Queryable, PooledConn};
use rusqlite::Error as RusqliteError;

use log::info;
use std::fmt::Error;

/// Inserts rows into a MySql table.
///
/// This function takes a `PooledConn` and a vector of objects implementing
/// the `Table` trait, which represents rows to be inserted into the table.
/// It generates SQL INSERT statements for each row and executes them within
/// a transaction.
///
/// # Arguments
///
/// * `conn` - A `PooledConn` to the MySql database.
/// * `table_rows` - A vector of objects implementing the `Table` trait representing
///                  the rows to be inserted into the database.
///
/// # Returns
///
/// A `Result` containing a `String` representing the joined SQL statements
/// if the insertion is successful, or a `RusqliteError` if an error occurs.
pub fn insert<T: Table>(
    conn: &mut PooledConn,
    table_rows: Vec<T>,
) -> Result<String, RusqliteError> {
    let mut statements: Vec<String> = Vec::new();
    for (index, table_row) in table_rows.iter().enumerate() {
        match generate_statement(table_row, index == 0) {
            Ok(statement) => statements.push(statement),
            Err(_) => return Err(RusqliteError::InvalidQuery),
        }
    }

    let joined_statements = statements.join(", ");

    // FIXME: Convert to transaction
    match conn.query_drop(&joined_statements) {
        Ok(_) => info!("Inserted into table, done."),
        Err(err) => eprintln!("Error: {}", err),
    }

    info!("Inserted into table, done.");

    // FIXME: Return the number of rows affected
    Ok(joined_statements)
}

/// Generates an SQL INSERT INTO statement for a given table row.
///
/// # Arguments
///
/// * `table_row` - A reference to an object implementing the `Table` trait.
///
/// # Returns
///
/// A `Result` containing a `String` representing the generated SQL statement
/// if the operation is successful, or a `RusqliteError` if an error occurs.
pub fn into<'a, T: Table + Default>(
    conn: &'a mut PooledConn,
    columns: Vec<String>,
    subquery: Box<dyn QueryBuilder<'a> + 'a>,
) -> Result<String, RusqliteError> {
    let statement = generate_insert_into_statement::<T>(columns, subquery);
    let sql = statement.unwrap();

    // FIXME: Convert to transaction
    let _ = conn.query_drop(&sql);

    info!("Inserted into table, done.");

    // FIXME: Return the number of rows affected
    return Ok(sql);
}

/// Generates an SQL INSERT INTO statement for a given subquery.
///
/// # Arguments
///
/// * `columns` - A `Vec` of column names.
/// * `subquery` - A `QueryBuilder` object representing the subquery.
///
/// # Returns
///
/// A `Result` containing a `String` representing the generated SQL statement
/// if the operation is successful, or a `RusqliteError` if an error occurs.
fn generate_insert_into_statement<'a, T: Table + Default>(
    columns: Vec<String>,
    subquery: Box<dyn QueryBuilder<'a> + 'a>,
) -> Result<String, RusqliteError> {
    let columns_str = columns.join(", ");
    let subquery_str = subquery.to_sql();
    let table_row = T::default();
    let table_name = table_row.get_name().replace("\"", "").replace("\\", "");

    let sql = format!(
        "INSERT INTO {} ({}) {}",
        table_name, columns_str, subquery_str
    );

    Ok(sql)
}

/// Generates an SQL INSERT statement for a given table row.
///
/// This function takes an object implementing the `Table` trait, representing
/// a single row of data to be inserted into the database. It generates an SQL
/// INSERT statement based on the column names and values of the table row.
///
/// # Arguments
///
/// * `table_row` - An object implementing the `Table` trait representing
///                 a single row of data to be inserted.
/// * `first_statement` - A boolean flag indicating whether this is the first
///                       statement to be generated.
///
/// # Returns
///
/// A `Result` containing a `String` representing the generated SQL statement
/// if successful, or a `Error` if an error occurs during the generation process.
fn generate_statement<T: Table>(table_row: &T, first_statement: bool) -> Result<String, Error> {
    // Generate strings for columns and values
    let mut columns_str = String::new();
    let mut values_str = String::new();

    // Iterate over the fields to generate columns and values
    let column_fields = table_row.get_column_fields();
    let column_values = table_row.get_column_values();

    for (column_name, value) in column_fields.iter().zip(column_values.iter()) {
        // Check if the field is an AutoIncrementPrimaryKey
        if table_row.is_auto_increment_primary_key(value) {
            println!("Skipping AutoIncrementPrimaryKey field in SQL statement generation.");
            continue;
        }
        columns_str.push_str(&format!("{}, ", column_name));
        values_str.push_str(&format!("'{}', ", value)); // Surround values with single quotes
    }

    // Sanitize table name from unwanted quotations or backslashes
    let table_name = table_row.get_name().replace("\"", "").replace("\\", "");

    // Remove the trailing comma and space
    if !columns_str.is_empty() {
        columns_str.pop();
        columns_str.pop();
    }
    if !values_str.is_empty() {
        values_str.pop();
        values_str.pop();
    }

    let sql = if first_statement {
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns_str, values_str
        )
    } else {
        format!("({})", values_str)
    };

    println!("{}", sql); // For debugging purposes

    Ok(sql)
}
