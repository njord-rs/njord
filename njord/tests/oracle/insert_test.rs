use super::User;
use njord::keys::AutoIncrementPrimaryKey;
use njord::oracle;
use std::vec;

#[test]
fn insert_row() {
    let connection_string = "//localhost:1521/FREEPDB1";
    let mut conn = oracle::open("njord_user", "njord_password", connection_string);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = oracle::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn insert_row_with_single_quotes() {
    let connection_string = "//localhost:1521/FREEPDB1";
    let mut conn = oracle::open("njord_user", "njord_password", connection_string);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "quote_user".to_string(),
        email: "quote_user@example.com".to_string(),
        address: "Some Random 'Address' 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = oracle::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
