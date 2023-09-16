use rusqlite::{Connection, Result};
use std::env;

pub trait Table {}

// initialize database with tables
pub fn init(tables: Vec<Box<dyn Table>>) -> Result<()> {
    let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "./target".to_string());
    let db_file_name = "my_database.db";
    let db_file_path = format!("{}/{}", target_dir, db_file_name);
    let conn = Connection::open(&db_file_path)?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;

    Ok(())
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
    struct TableA {
        data: i32,
    }

    struct TableB {
        data: f64,
    }

    impl Table for TableA {}
    impl Table for TableB {}

    let a = TableA { data: 42 };
    let b = TableB { data: 3.14 };

    let tables: Vec<Box<dyn Table>> =
        vec![Box::new(a) as Box<dyn Table>, Box::new(b) as Box<dyn Table>];

    let result = super::init(tables);

    assert!(result.is_ok());
}
