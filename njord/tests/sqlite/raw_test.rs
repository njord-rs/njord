use std::path::Path;

use njord::sqlite;
use njord_derive::sql;

#[test]
fn execute_raw_sql() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path).unwrap();

    let sql = sql! {
        INSERT INTO users (username, email, address)
        VALUES
            ("raw_execute", "raw_execute@icloud.com", "raw_execute_address");

        DELETE FROM users
        WHERE username = "raw_execute";
    };

    let results = sqlite::raw_execute(&conn, &sql);

    assert!(!results.is_err());
}
