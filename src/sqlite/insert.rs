use crate::sqlite::TableDefinition;

use super::init::open;
use super::Table;
use log::info;
use rusqlite::{Connection, Error, Result};
use std::{collections::HashMap, env};

pub fn insert(table: &dyn Table) -> Result<()> {
    let mut conn = open()?;

    // create a transaction
    let tx = conn.transaction()?;

    // let statement = generate_statement(&**table);
    // tx.execute(&statement, [])?;

    // commit the transaction
    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(())
}

// might need to re-think this one
/* fn generate_statement(table: &dyn Table, values: Vec<&str>) -> Result<String, Error> {
    // second parameter for values?

    let mut columns_str = String::new();
    for column_name in table.get_column_fields() {
        columns_str.push_str(&format!("{}, ", column_name));
    }

    let mut values_str = String::new();
    for value in values {
        values_str.push_str(&format!("{}, ", value))
    }

    // remove the trailing comma and space
    columns_str.pop();
    columns_str.pop();
    values_str.pop();
    values_str.pop();

    let sql = format!(
        "INSERT INTO {} (
            {}
        )
        VALUES (
            {}
        )",
        table.get_name(),
        columns_str,
        values_str
    );

    Ok(sql)
}

#[test]
fn test_insert() {
    let table1_values = vec!["New ORM library for Rust"];
    let table2_values = vec!["Rust is a great language!"];

    // create the posts table
    let posts = TableDefinition {
        name: "posts".to_string(),
        columns: {
            let mut map = HashMap::new();
            map.insert("title".to_string(), "TEXT NOT NULL".to_string());
            map
        },
    };

    // create the categories table
    let categories = TableDefinition {
        name: "categories".to_string(),
        columns: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "TEXT NOT NULL".to_string());
            map
        },
    };

    assert!(result_posts.is_ok());
    assert!(result_categories.is_ok());
} */
