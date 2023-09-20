use crate::table::Table;
use log::info;
use njord_derive::Table;
use rusqlite::{Connection, Error, Result};
use std::{collections::HashMap, env};

use crate::sqlite::open;

#[derive(Table)]
struct Hello {
    name: String,
    columns: HashMap<String, String>,
}

// initialize database with tables
pub fn init(tables: Vec<Box<dyn Table>>) -> Result<()> {
    // let test = Hello {
    //     name: "".to_string(),
    //     columns: HashMap::new(),
    // };

    let mut conn = open()?;

    // create a transaction
    let tx = conn.transaction()?;

    // execute all sql statements based on tables vector parameter
    for t in &tables {
        let statement = generate_statement(&**t);
        tx.execute(&statement, [])?;
    }

    // commit the transaction
    tx.commit()?;

    info!("Initialize database, done.");

    Ok(())
}

// generate sql statement for create table
fn generate_statement(table: &dyn Table) -> String {
    // generate the column definitions based on the hashmap
    let mut column_definitions = String::new();
    for (column_name, column_type) in table.get_columns() {
        column_definitions.push_str(&format!("{} {}, ", column_name, column_type));
    }

    // remove the trailing comma and space
    column_definitions.pop();
    column_definitions.pop();

    let sql = format!(
        "CREATE TABLE {} (id INTEGER PRIMARY KEY, {});",
        table.get_name(),
        column_definitions
    );

    println!("{}", sql);

    sql
}

// #[test]
// fn test_init() {
//     // create the posts table
//     let posts = TableDefinition {
//         name: "posts".to_string(),
//         columns: {
//             let mut map = HashMap::new();
//             map.insert("title".to_string(), "TEXT NOT NULL".to_string());
//             map.insert("comments".to_string(), "INT DEFAULT 0".to_string());
//             map
//         },
//     };

//     // create the categories table
//     let categories = TableDefinition {
//         name: "categories".to_string(),
//         columns: {
//             let mut map = HashMap::new();
//             map.insert("name".to_string(), "TEXT NOT NULL".to_string());
//             map.insert("posts_amount".to_string(), "INT DEFAULT 0".to_string());
//             map
//         },
//     };

//     // initialize a vector of the tables to create
//     let tables: Vec<Box<dyn Table>> = vec![
//         Box::new(posts) as Box<dyn Table>,
//         Box::new(categories) as Box<dyn Table>,
//     ];

//     let result = init(tables);

//     assert!(result.is_ok());
// }
