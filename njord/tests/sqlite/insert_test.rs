use super::User;
use njord::column::Column;
use njord::condition::{Condition, Value};
use njord::keys::AutoIncrementPrimaryKey;
use njord::sqlite;
use std::path::Path;
use std::vec;

#[test]
fn insert_row() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let mut conn = sqlite::open(db_path);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = sqlite::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn insert_with_sub_query() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let mut conn = sqlite::open(db_path);

    match conn {
        Ok(ref mut c) => {
            let subquery = sqlite::select(
                vec![
                    Column::Text("username".to_string()),
                    Column::Text("email".to_string()),
                    Column::Text("address".to_string()),
                ],
            )
            .from(User::default())
            .where_clause(Condition::Eq(
                "username".to_string(),
                Value::Literal("mjovanc".to_string()),
            ));

            let result = sqlite::insert::into::<User>(
                c,
                vec![
                    "username".to_string(),
                    "email".to_string(),
                    "address".to_string(),
                ],
                Box::new(subquery),
            );
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn insert_row_with_single_quotes() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let mut conn = sqlite::open(db_path);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "quote_user".to_string(),
        email: "quote_user@example.com".to_string(),
        address: "Some Random 'Address' 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = sqlite::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
