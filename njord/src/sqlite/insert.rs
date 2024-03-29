use crate::table::Table;
use crate::util::convert_insert_values;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

pub fn insert<T: Table>(mut conn: Connection, table_rows: &Vec<T>) -> Result<()> {
    // create a transaction
    let tx = conn.transaction()?;
    let mut statement = String::from("");

    for row in table_rows {
        match generate_statement(row) {
            Ok(r) => statement.push_str(r.as_str()),
            Err(e) => panic!("Problem generating statement: {:?}.", e)
        }
    }

    tx.execute(statement.as_str(), [])?;

    // commit the transaction
    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(())
}

fn generate_statement<T: Table>(table_row: &T) -> Result<String, Error> {
    // generate string for columns
    let mut columns_str = String::new();
    for column_name in table_row.get_column_fields() {
        columns_str.push_str(&format!("{}, ", column_name));
    }

    // surround single quotes of text
    let converted_values = convert_insert_values(table_row.get_column_values());

    // generate values string
    let mut values_str = String::new();
    for value in converted_values {
        let data_type_str = value.to_string();
        values_str.push_str(&data_type_str);
        values_str.push_str(", ");
    }

    // sanitize table name from unwanted quotations or backslashes
    let table_name = table_row.get_name().replace("\"", "").replace("\\", "");

    // remove the trailing comma and space
    columns_str.pop();
    columns_str.pop();
    values_str.pop();
    values_str.pop();

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({}); ",
        table_name,
        columns_str,
        values_str
    );

    println!("{}", sql);

    Ok(sql)
}
