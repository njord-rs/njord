use super::User;
use njord::condition::{Condition, Value};
use njord::keys::AutoIncrementPrimaryKey;
use njord::mssql;
use std::vec;

#[tokio::test]
async fn delete_row() {
    insert_row().await;

    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    match conn {
        Ok(ref mut c) => {
            let result = mssql::delete(c)
                .from(User::default())
                .where_clause(Condition::Eq(
                    "username".to_string(),
                    Value::Literal("chasewillden2".to_string()),
                ))
                .build()
                .await;
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

/// Helper function to insert a row to be deleted
async fn insert_row() {
    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden2".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = mssql::insert(c, vec![table_row]).await;
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}
