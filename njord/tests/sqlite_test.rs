// integrations tests for sqlite

use njord::sqlite;
use njord::table::Table;
use njord_derive::Table;
use serial_test::serial;

mod common;

#[test]
#[serial]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
#[serial]
fn init_tables() {
    let conn = sqlite::open("test_database.db");

    let tables = common::initialized_tables_sqlite();

    let result = sqlite::init(conn.unwrap(), tables);

    assert!(result.is_ok());
}

// #[test]
// #[serial]
// fn insert_row() {
//     let conn = sqlite::open("test_database.db");

//     let tables = common::initialized_tables_sqlite();
//     let table_a: &Box<dyn Table> = &tables[0];

//     let table_row: TableA = TableA {
//         title: "Table A".to_string(),
//         description: "Some description for Table A".to_string(),
//         amount: 0,
//     };

//     let result = sqlite::insert(conn.unwrap(), &table_row);

//     assert!(result.is_ok());
// }

#[test]
#[serial]
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

    let result = sqlite::drop(conn, &TableA::default());

    println!("Result: {:?}", result);

    assert!(result.is_ok());
}
