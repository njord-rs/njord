// integrations tests for sqlite

use std::collections::HashMap;
use njord::sqlite::{self, Condition};
use njord::table::Table;

#[cfg(feature = "derive")]
use njord_derive::Table;
use crate::common::print_rows;

mod common;

#[derive(Table, Debug, Default)]
struct TableA {
    title: String,
    description: String,
    amount: u32,
}

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

    let table_row: TableA = TableA {
        title: "Table A".to_string(),
        description: "Some description for Table A".to_string(),
        amount: 0,
    };

    match init_tables_result {
        Ok(_) => {
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
            let result = sqlite::drop_table(conn, &TableA::default());
            assert!(result.is_ok());
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
    common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

    let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
    let condition = Condition::Eq(
        "description".to_string(),
        "Some description for Table A".to_string(),
    );

    match init_tables_result {
        Ok(_) => {
            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(condition)
                .build();

            match result {
                Ok(result) => {
                    println!("\nSELECT ROWS: ");
                    print_rows(&result);
                    assert_eq!(result.len(), 2);
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}

#[test]
fn select_distinct() {
    let db_name = "select_distinct.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);
    common::insert_rows_sqlite_distinct(db_name).expect("Failed to insert rows to sqlite.");

    let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
    let condition = Condition::Eq(
        "description".to_string(),
        "Some description for Table A".to_string(),
    );

    match init_tables_result {
        Ok(_) => {
            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(condition)
                .distinct()
                .build();

            match result {
                Ok(result) => {
                    println!("\nSELECT DISTINCT ROWS: ");
                    print_rows(&result);
                    assert_eq!(result.len(), 1);
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}

#[test]
fn select_group_by() {
    let db_name = "select_group_by.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);
    common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

    let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
    let condition = Condition::Eq(
        "description".to_string(),
        "Some description for Table A".to_string(),
    );
    let group_by = vec![
        "description".to_string(),
        "amount".to_string(),
    ];

    match init_tables_result {
        Ok(_) => {
            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(condition)
                .group_by(group_by)
                .build();

            match result {
                Ok(result) => {
                    println!("\nSELECT GROUP BY ROWS: ");
                    print_rows(&result);
                    assert_eq!(result.len(), 2);
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}

#[test]
fn select_order_by() {
    let db_name = "select_order_by.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);
    common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

    let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
    let condition = Condition::Eq(
        "description".to_string(),
        "Some description for Table A".to_string(),
    );
    let group_by = vec![
        "description".to_string(),
        "amount".to_string(),
    ];
    let mut order_by = HashMap::new();
    order_by.insert(vec!["amount".to_string()], "DESC".to_string());
    order_by.insert(vec!["description".to_string()], "ASC".to_string());

    match init_tables_result {
        Ok(_) => {
            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .build();

            match result {
                Ok(result) => {
                    println!("\nSELECT ORDER BY ROWS: ");
                    print_rows(&result);
                    assert_eq!(result.len(), 2);
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}

#[test]
fn select_limit_offset() {
    let db_name = "select_limit_offset.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);
    common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

    let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
    let condition = Condition::Eq(
        "description".to_string(),
        "Some description for Table A".to_string(),
    );
    let group_by = vec![
        "description".to_string(),
        "amount".to_string(),
    ];
    let mut order_by = HashMap::new();
    order_by.insert(vec!["amount".to_string()], "ASC".to_string());
    order_by.insert(vec!["description".to_string()], "DESC".to_string());

    match init_tables_result {
        Ok(_) => {
            //TODO we should probably get back a vector of the table that was used so we can more
            // easily pass around that struct in the code
            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .limit(1)
                .offset(1)
                .build();

            match result {
                Ok(result) => {
                    println!("\nSELECT LIMIT & OFFSET ROWS: ");
                    // print_rows(&result);
                    assert_eq!(result.len(), 1);
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}

#[test]
fn select_having() {
    let db_name = "select_having.db";
    let _ = common::drop_db_sqlite(db_name);
    let conn = common::open_db_sqlite(db_name).unwrap();
    let init_tables_result = common::initialize_tables_sqlite(db_name);
    common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

    let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
    let where_condition = Condition::Eq(
        "description".to_string(),
        "Some description for Table A".to_string(),
    );
    let group_by = vec![
        "description".to_string(),
        "amount".to_string(),
    ];
    let mut order_by = HashMap::new();
    order_by.insert(vec!["amount".to_string()], "ASC".to_string());
    order_by.insert(vec!["description".to_string()], "DESC".to_string());
    let having_condition = Condition::Gt(
        "amount".to_string(),
        "10".to_string(),
    );

    match init_tables_result {
        Ok(_) => {
            let result = sqlite::select(conn, columns)
                .from(&TableA::default())
                .where_clause(where_condition)
                .order_by(order_by)
                .group_by(group_by)
                .having(having_condition)
                .build();

            match result {
                Ok(result) => {
                    println!("\nSELECT HAVING: ");
                    print_rows(&result);
                    assert_eq!(result.len(), 1);
                },
                Err(error) => panic!("Failed to SELECT: {:?}", error),
            };
        }
        Err(error) => panic!("Failed to select: {:?}", error),
    };
}
