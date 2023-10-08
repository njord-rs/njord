// integrations tests for sqlite

use njord::sqlite;
use njord::table::Table;
use njord_derive::Table;
use serial_test::file_serial;

mod common;

#[test]
#[file_serial]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
#[file_serial]
fn init_tables() {
    let conn = sqlite::open("test_database.db");

    let tables = common::initialized_tables_sqlite();

    let result = sqlite::init(conn.unwrap(), tables);

    assert!(result.is_ok());
}

#[test]
#[file_serial]
fn insert_row() {
    let conn = sqlite::open("test_database.db");

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

    let result = sqlite::insert(conn.unwrap(), &table_row);

    assert!(result.is_ok());
}

#[test]
#[file_serial]
fn drop_table() {
    let conn = sqlite::open("test_database.db").unwrap();

    let tables = common::initialized_tables_sqlite();

    let _ = sqlite::init(conn, tables);

    #[derive(Table, Debug, Default)]
    struct TableA {
        title: String,
        description: String,
        amount: u32,
    }

    let conn = sqlite::open("test_database.db").unwrap();

    let result = sqlite::drop_table(conn, &TableA::default());

    println!("Result: {:?}", result);

    assert!(result.is_ok());
}
