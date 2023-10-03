// integrations tests for sqlite

use njord::sqlite;

#[test]
fn open_db() {
    let result = sqlite::open("test_database.db");
    assert!(result.is_ok());
}

#[test]
fn init_tables() {
    // open a new db
    assert_eq!(true, true);
}

#[test]
fn insert_row() {
    // open a new db
    // init tables
    assert_eq!(true, true);
}
