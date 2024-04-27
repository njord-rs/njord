use crate::table::Table;
use crate::util::convert_insert_values;

use rusqlite::Error as RusqliteError;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

pub fn insert<T: Table>(mut conn: Connection, table_rows: Vec<T>) -> Result<String, RusqliteError> {
    // Create a transaction
    let tx = conn.transaction()?;

    // Accumulate statements
    let mut statements: Vec<String> = Vec::new();
    for table_row in table_rows {
        match generate_statement(&table_row) {
            Ok(statement) => statements.push(statement),
            Err(_) => return Err(RusqliteError::InvalidQuery),
        }
    }

    // Join statements into a single string
    let joined_statements = statements.join("; ");

    // Execute all statements at once
    tx.execute_batch(&joined_statements)?;

    // Commit the transaction
    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(joined_statements)
}

fn generate_statement<T: Table>(table_row: &T) -> Result<String, Error> {
    // Generate string for columns
    let mut columns_str = String::new();
    for column_name in table_row.get_column_fields() {
        // TODO: filter out id field
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
