use njord::sqlite;
use std::path::Path;

#[test]
fn open_db() {
    let db_relative_path = "./db/open.db";
    let db_path = Path::new(&db_relative_path);

    let result = sqlite::open(db_path);
    assert!(result.is_ok());
}