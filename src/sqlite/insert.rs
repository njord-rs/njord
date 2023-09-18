use crate::table::{Table, TableDefinition};
use crate::util::convert_insert_values;

use super::init::open;
use log::info;
use rusqlite::Result;
use std::collections::HashMap;
use std::fmt::Error;

pub fn insert(table: &dyn Table, values: Vec<&str>) -> Result<()> {
    let mut conn = open()?;

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
    let mut converted_values = convert_insert_values(values);

    // generate values string
    let mut values_str = String::new();
    for value in converted_values {
        let data_type_str = value.to_string();
        values_str.push_str(&*data_type_str);
        values_str.push_str(", ");
    };

    // remove the trailing comma and space
    columns_str.pop();
    columns_str.pop();
    values_str.pop();
    values_str.pop();

    let sql = format!("INSERT INTO {} ({}) VALUES ({});",
        table.get_name(),
        columns_str,
        values_str
    );

    println!("{}", sql);

    Ok(sql)
}

#[test]
fn test_insert() {
    let table1_values = vec!["New ORM library for Rust", "10"];
    let table2_values = vec!["Rust is a great language!", "5"];

    // create the posts table
    let posts = TableDefinition {
        name: "posts".to_string(),
        columns: {
            let mut map = HashMap::new();
            map.insert("title".to_string(), "TEXT NOT NULL".to_string());
            map.insert("comments".to_string(), "INT DEFAULT 0".to_string());
            map
        },
    };

    // create the categories table
    let categories = TableDefinition {
        name: "categories".to_string(),
        columns: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "TEXT NOT NULL".to_string());
            map.insert("posts_amount".to_string(), "INT DEFAULT 0".to_string());
            map
        },
    };

    let result_posts = insert(&posts, table1_values);
    let result_categories = insert(&categories, table2_values);

    assert!(result_posts.is_ok());
    assert!(result_categories.is_ok());
}
