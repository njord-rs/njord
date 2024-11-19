use super::User;
use njord::keys::AutoIncrementPrimaryKey;
use njord::mariadb;
use std::vec;

#[test]
fn insert_row() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn insert_row_with_single_quotes() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "quote_user".to_string(),
        email: "quote_user@example.com".to_string(),
        address: "Some Random 'Address' 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
