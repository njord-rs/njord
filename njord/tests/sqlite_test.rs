// integrations tests for sqlite

use njord::sqlite;
use njord_derive::Table;

#[test]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
fn init_tables() {
    // open a new db

    // create table 1
    #[derive(Table, Debug)]
    struct TableA {
        title: String,
        desc: String,
        amount: u32,
    }

    let table_a = TableA {
        title: "Table A".to_string(),
        desc: "Some description for Table A".to_string(),
        amount: 0,
    };

    // create table 2
    // #[derive(Table, Debug)]
    // struct TableB {
    //     title: String,
    //     desc: String,
    //     amount: u32,
    // }

    // let table_b = TableB {
    //     title: "Table B".to_string(),
    //     desc: "Some description for Table A".to_string(),
    //     amount: 0,
    // };

    // create table 3

    println!("Name of Table: {}", table_a.get_name());
    println!("Columns of Table: {:?}", table_a.get_columns());
    println!("Column Fields of Table: {:?}", table_a.get_column_fields());

    assert_eq!(true, true);
}

#[test]
fn insert_row() {
    // open a new db
    // init tables
    assert_eq!(true, true);
}
