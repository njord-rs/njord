use njord::oracle;

#[test]
fn open_db() {
    let connection_string = "//localhost:1521/FREEPDB1";
    let conn = oracle::open("njord_user", "njord_password", connection_string);
    assert!(conn.is_ok());
}
