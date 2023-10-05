// integrations tests for sqlite

use njord::sqlite;
use njord::sqlite::init::init;
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
    // open a new db
    //common::setup_sqlite();
    let result = sqlite::open("test_database.db");

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

    let table_a = Box::new(TableA::default());
    let table_b = Box::new(TableB::default());
    let table_c = Box::new(TableC::default());

    let mut tables: Vec<Box<dyn Table>> = Vec::new();
    tables.push(table_a);
    tables.push(table_b);
    tables.push(table_c);

    let result = init(tables);

    assert!(result.is_ok());

    assert_eq!(true, true);
}

#[test]
fn insert_row() {
    // open a new db
    // init tables
    assert_eq!(true, true);
}
