use njord::mariadb;

#[test]
fn open_db() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let result = mariadb::open(url);
    assert!(result.is_ok());
}
