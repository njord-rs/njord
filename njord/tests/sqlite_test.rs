// integrations tests for sqlite

use log::info;
use rusqlite::types::Value;
use njord::sqlite::{self, Condition};
use njord::table::Table;

#[cfg(feature = "derive")]
use njord_derive::Table;

mod common;

#[test]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
fn init_tables() {
    let db_name = "init_tables.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();

    let tables = common::generate_tables_sqlite();

    let result = sqlite::init(conn, tables);

    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let db_name = "insert_row.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);

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

            assert!(result.is_ok());
        }
        Err(error) => panic!("Failed to insert row: {:?}", error),
    };
}

#[test]
fn drop_table() {
    let db_name = "drop_table.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);

    match init_tables_result {
        Ok(_) => {
            #[derive(Table, Debug, Default)]
            struct TableA {
                title: String,
                description: String,
                amount: u32,
            }

            let result = sqlite::drop_table(conn, &TableA::default());

            assert!(result.is_ok());
            // assert_eq!(result.iter().len(), 2);
        }
        Err(error) => panic!("Failed to drop table: {:?}", error),
    }
}

#[test]
fn select() {
    let db_name = "select.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);
    let insert_rows_result = common::insert_rows_sqlite(db_name);

    match init_tables_result {
        Ok(_) => {
            #[derive(Table, Debug, Default)]
            struct TableA {
                title: String,
                description: String,
                amount: u32,
            }
            let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
            let condition = Condition::Eq(
                "description".to_string(),
                "Some description for Table A".to_string(),
            );

            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(condition)
                .build();

            // currently returns error with "ExecuteReturnedResults"
            match result {
                Ok(result) => {
                    println!("\nRETURNED ROWS: ");
                    for row in result {
                        for (column, value) in &row {
                            match value {
                                Value::Null => println!("\t{}: NULL", column),
                                Value::Integer(i) => println!("\t{}: {}", column, i),
                                Value::Real(f) => println!("\t{}: {}", column, f),
                                Value::Text(s) => println!("\t{}: {}", column, s),
                                Value::Blob(b) => println!("\t{}: <blob of length {}>", column, b.len()),
                            }
                        }
                        println!("\t---");
                    }
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}
