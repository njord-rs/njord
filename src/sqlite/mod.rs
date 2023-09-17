use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::{env, vec};

pub trait Table {
    fn get_name(&self) -> &str;
    fn get_columns(&self) -> &HashMap<String, String>;
}

pub struct TableStruct {
    name: String,
    columns: HashMap<String, String>,
}

impl Table for TableStruct {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_columns(&self) -> &HashMap<String, String> {
        return &self.columns;
    }
}

// initialize database with tables
pub fn init(tables: Vec<Box<dyn Table>>) -> Result<()> {
    let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "./target".to_string());
    let db_file_name = "my_database.db";
    let db_file_path = format!("{}/{}", target_dir, db_file_name);
    let conn = Connection::open(&db_file_path)?;

    // execute all sql statements based on tables vector parameter
    for t in &tables {
        let statement = convert_to_create_table_str(&**t);
        conn.execute(&statement, [])?;
    }

    Ok(())
}

fn convert_to_create_table_str(table: &dyn Table) -> String {
    // generate the column definitions based on the hashmap
    let mut column_definitions = String::new();
    for (column_name, column_type) in table.get_columns() {
        column_definitions.push_str(&format!("{} {}, ", column_name, column_type));
    }

    // remove the trailing comma and space
    column_definitions.pop();
    column_definitions.pop();

    let table_sql = format!(
        "CREATE TABLE {} (
            id   INTEGER PRIMARY KEY,
            {}
        )",
        table.get_name(),
        column_definitions
    );

    table_sql
}

// select
// Vector of strings of fields and struct name
// fn select() -> Result<()> {}

// update
// initilized struct (table) and condition as string as argument

// delete
// table struct and condition (string) as argument

// insert into
// insert new record/row into table
// "initilized" struct (table) as argument that contains column names and values

// create table
// should take a struct as argument so we know the column names and value types

#[test]
fn test_init() {
    // create the posts table
    let posts = TableStruct {
        name: "posts".to_string(),
        columns: {
            let mut map = HashMap::new();
            map.insert("title".to_string(), "TEXT NOT NULL".to_string());
            map
        },
    };

    // create the categories table
    let categories = TableStruct {
        name: "categories".to_string(),
        columns: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "TEXT NOT NULL".to_string());
            map
        },
    };

    let tables: Vec<Box<dyn Table>> = vec![
        Box::new(posts) as Box<dyn Table>,
        Box::new(categories) as Box<dyn Table>,
    ];

    let result = init(tables);

    assert!(result.is_ok());
}
