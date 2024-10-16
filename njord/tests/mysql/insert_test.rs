use super::User;
use njord::keys::AutoIncrementPrimaryKey;
use njord::mysql;
use std::vec;

#[test]
fn insert_row() {
    let url = "mysql://njord_user:njord_password@localhost:3306/njord_db";
    let mut conn = mysql::open(url);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = mysql::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
