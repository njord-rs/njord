// integrations tests for sqlite

use njord::sqlite;
use njord::sqlite::init::init;
use njord::sqlite::insert::insert;
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
    let conn = sqlite::open("test_database.db");

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

    let result = init(conn.unwrap(), tables);

    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let conn = sqlite::open("test_database.db");

    #[derive(Table, Debug, Default)]
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

    let result = insert(conn.unwrap(), &table_row);

    assert!(result.is_ok());
}
