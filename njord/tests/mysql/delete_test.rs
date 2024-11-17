use super::User;
use ::mysql::PooledConn;
use njord::condition::{Condition, Value};
use njord::keys::AutoIncrementPrimaryKey;
use njord::mysql;
use std::vec;

#[test]
fn delete_row() {
    let url = "mysql://njord_user:njord_password@localhost:3306/njord_db";
    let mut conn = mysql::open(url);

    match conn {
        Ok(ref mut c) => {
            insert_row(c);

            let result = mysql::delete()
                .from(User::default())
                .where_clause(Condition::Eq(
                    "username".to_string(),
                    Value::Literal("chasewillden2".to_string()),
                ))
                .build(c);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

/// Helper function to insert a row to be deleted
fn insert_row(conn: &mut PooledConn) {
    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden2".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    let result = mysql::insert(conn, vec![table_row]);
    assert!(result.is_ok());
}
