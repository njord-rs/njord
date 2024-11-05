use njord::mssql;

#[tokio::test]
async fn open_db() {
    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let conn = mssql::open(connection_string).await;
    assert!(conn.is_ok());
}
