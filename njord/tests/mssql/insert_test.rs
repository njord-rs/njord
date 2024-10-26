use super::User;
use njord::keys::AutoIncrementPrimaryKey;
use njord::mssql;
use std::vec;

#[tokio::test]
async fn insert_row() {
    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=njord_password;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden".to_string(),
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
