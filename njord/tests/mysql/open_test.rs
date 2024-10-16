use njord::mysql;

#[test]
fn open_db() {
    let url = "mysql://njord_user:njord_password@localhost:3306/njord_db";
    let result = mysql::open(url);
    assert!(result.is_ok());
}
