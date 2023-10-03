use crate::sqlite::open;
use crate::table::Table;
use crate::util::convert_insert_values;

use log::info;
use rusqlite::Result;
use std::collections::HashMap;
use std::fmt::Error;

pub fn insert(table: &dyn Table, values: Vec<&str>) -> Result<()> {
    let mut conn = open("my_database.db")?;

    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement(table, values);
    tx.execute(&statement.unwrap(), [])?;

    // commit the transaction
    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(())
}

fn generate_statement(table: &dyn Table, values: Vec<&str>) -> Result<String, Error> {
    // generate string for columns
    let mut columns_str = String::new();
    for column_name in table.get_column_fields() {
        columns_str.push_str(&format!("{}, ", column_name));
    }

    // surround single quotes of text
    let converted_values = convert_insert_values(values);

    // generate values string
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
        table.get_name(),
        columns_str,
        values_str
    );

    println!("{}", sql);

    Ok(sql)
}
