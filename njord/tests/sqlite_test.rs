// integrations tests for sqlite

use njord::sqlite;
use njord::table::Table;
use njord_derive::Table;

mod common;

#[test]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
fn init_tables() {
    let _ = common::drop_db_sqlite();
    let conn = common::open_db_sqlite("init_tables.db").unwrap();

    let tables = common::initialized_tables_sqlite();

    let result = sqlite::init(conn, tables);

    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let _ = common::drop_db_sqlite();
    let conn = common::open_db_sqlite("insert_row.db").unwrap();
    let init_tables_result = common::initialize_tables_sqlite("insert_row.db");

    match init_tables_result {
        Ok(_) => {
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
        }
        Err(error) => panic!("Failed to insert row: {:?}", error),
    };
}

#[test]
fn drop_table() {
    let _ = common::drop_db_sqlite();
    let conn = common::open_db_sqlite("drop_table.db").unwrap();
    let init_tables_result = common::initialize_tables_sqlite("drop_table.db");

    match init_tables_result {
        Ok(_) => {
            #[derive(Table, Debug, Default)]
            struct TableA {
                title: String,
                description: String,
                amount: u32,
            }

            let result = sqlite::drop_table(conn, &TableA::default());

            println!("Result: {:?}", result);

            assert!(result.is_ok());
        }
        Err(error) => panic!("Failed to drop table: {:?}", error),
    }
}
