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

use crate::table::Table;
use crate::util::convert_insert_values;

use rusqlite::Error as RusqliteError;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

/// Inserts rows into a SQLite table.
///
/// This function takes a `Connection` and a vector of objects implementing
/// the `Table` trait, which represents rows to be inserted into the table.
/// It generates SQL INSERT statements for each row and executes them within
/// a transaction.
///
/// # Arguments
///
/// * `conn` - A `Connection` to the SQLite database.
/// * `table_rows` - A vector of objects implementing the `Table` trait representing
///                  the rows to be inserted into the database.
///
/// # Returns
///
/// A `Result` containing a `String` representing the joined SQL statements
/// if the insertion is successful, or a `RusqliteError` if an error occurs.
pub fn insert<T: Table>(mut conn: Connection, table_rows: Vec<T>) -> Result<String, RusqliteError> {
    let tx: rusqlite::Transaction<'_> = conn.transaction()?;

    let mut statements: Vec<String> = Vec::new();
    for table_row in table_rows {
        match generate_statement(&table_row) {
            Ok(statement) => statements.push(statement),
            Err(_) => return Err(RusqliteError::InvalidQuery),
        }
    }

    let joined_statements = statements.join("; ");

    tx.execute_batch(&joined_statements)?;

    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(joined_statements)
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
///
/// # Returns
///
/// A `Result` containing a `String` representing the generated SQL statement
/// if successful, or a `Error` if an error occurs during the generation process.
fn generate_statement<T: Table>(table_row: &T) -> Result<String, Error> {
    // Generate string for columns
    let mut columns_str = String::new();
    for column_name in table_row.get_column_fields() {
        columns_str.push_str(&format!("{}, ", column_name));
    }

    // Surround single quotes of text
    let converted_values = convert_insert_values(table_row.get_column_values());

    // Generate values string
    let mut values_str = String::new();
    for value in converted_values {
        let data_type_str = value.to_string();
        values_str.push_str(&data_type_str);
        values_str.push_str(", ");
    }

    // Sanitize table name from unwanted quotations or backslashes
    let table_name = table_row.get_name().replace("\"", "").replace("\\", "");

    // Remove the trailing comma and space
    columns_str.pop();
    columns_str.pop();
    values_str.pop();
    values_str.pop();

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({}); ",
        table_name, columns_str, values_str
    );

    println!("{}", sql);

    Ok(sql)
}
