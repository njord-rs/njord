use std::{env, fs, vec};

use njord::{sqlite, table::Table};

#[cfg(feature = "derive")]
use njord_derive::Table;
use rusqlite::{Connection, Error};

pub fn open_db_sqlite(db_name: &str) -> Result<Connection, Error> {
    let conn = sqlite::open(db_name).unwrap();
    Ok(conn)
}

pub fn drop_db_sqlite(db_name: &str) -> Result<(), std::io::Error> {
    let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "../target".to_string());
    let db_file_path = format!("{}/{}", target_dir, db_name);
    fs::remove_file(db_file_path)
}

pub fn initialize_tables_sqlite(db_name: &str) -> Result<(), Error> {
    let conn = open_db_sqlite(db_name).unwrap();

    let tables = generate_tables_sqlite();

    let result = sqlite::init(conn, tables);

    result
}

pub fn insert_rows_sqlite(db_name: &str) -> Result<(), Error> {
    let conn = open_db_sqlite(db_name)?;

    #[derive(Table, Debug)]
    struct TableA {
        title: String,
        description: String,
        amount: u32,
    }

    let table_row: TableA = TableA {
        title: "Table A".to_string(),
        description: "Some description for Table A".to_string(),
        amount: 0,
    };

    let result = sqlite::insert(conn, &table_row);

    println!("Result: {:?}", result);

    assert!(result.is_ok());

    result
}

pub fn generate_tables_sqlite() -> Vec<Box<dyn Table>> {
    #[derive(Table, Debug, Default)]
    struct TableA {
        title: String,
        description: String,
        amount: u32,
    }

    #[derive(Table, Debug, Default)]
    struct TableB {
        name: String,
        age: u32,
        email: String,
    }

    #[derive(Table, Debug, Default)]
    struct TableC {
        product_id: i64,
        product_name: String,
        price: f64,
        in_stock: bool,
    }

    let table_a = Box::<TableA>::default();
    let table_b = Box::<TableB>::default();
    let table_c = Box::<TableC>::default();

    let tables: Vec<Box<dyn Table>> = vec![table_a, table_b, table_c];

    tables
}
