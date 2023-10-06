use crate::sqlite::open;
use crate::table::Table;
use crate::util::convert_insert_values;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

// change table -> table_row and remove values later
// we will take in an intialized struct for the tables
// for example we want to send in
//
// let table_row = Box::new(TableA {
//    title: "Some value here".to_string(),
//    desc: "Some other value here".to_string(),
//    amount: 0,
// });
pub fn insert(mut conn: Connection, table_row: &dyn Table) -> Result<()> {
    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement(table_row);
    tx.execute(&statement.unwrap(), [])?;

    // commit the transaction
    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(())
}

fn generate_statement(table_row: &dyn Table) -> Result<String, Error> {
    // generate string for columns
    let mut columns_str = String::new();
    for column_name in table_row.get_column_fields() {
        columns_str.push_str(&format!("{}, ", column_name));
    }

    // surround single quotes of text
    let converted_values = convert_insert_values(table_row.get_column_values());

    // // generate values string
    let mut values_str = String::new();
    for value in converted_values {
        let data_type_str = value.to_string();
        values_str.push_str(&*data_type_str);
        values_str.push_str(", ");
    }

    // remove the trailing comma and space
    columns_str.pop();
    columns_str.pop();
    values_str.pop();
    values_str.pop();

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table_row.get_name(),
        columns_str,
        values_str
    );

    println!("SQL: {}", sql);

    Ok(sql)
}
