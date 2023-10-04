// integrations tests for sqlite

use njord::sqlite;
use njord::sqlite::init::init;
use njord::table::Table;
use njord_derive::Table;

#[test]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
fn init_tables() {
    // open a new db
    // common::setup_sqlite();

    // create table 1
    #[derive(Table, Debug)]
    struct TableA {
        title: String,
        desc: String,
        amount: u32,
    }

    let table_a = Box::new(TableA {
        title: "Table A".to_string(),
        desc: "Some description for Table A".to_string(),
        amount: 0,
    });

    // create table 2
    #[derive(Table, Debug)]
    struct TableB {
        name: String,
        age: u32,
        email: String,
    }

    let table_b = Box::new(TableB {
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    });

    // create table 3
    #[derive(Table, Debug)]
    struct TableC {
        product_id: i64,
        product_name: String,
        price: f64,
        in_stock: bool,
    }

    let table_c = Box::new(TableC {
        product_id: 1001,
        product_name: "Example Product".to_string(),
        price: 29.99,
        in_stock: true,
    });

    // add a vector of the tables here
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
