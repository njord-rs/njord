// insert new record/row into specific table
pub fn insert(table: &dyn Table) -> Result<()> {
    let mut conn = open()?;

    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement(&**table);
    tx.execute(&statement, [])?;

    // commit the transaction
    tx.commit()?;

    info!("Inserted into table, done.");

    Ok(())
}

fn generate_statement(table: &dyn Table) -> Result<String, Error> {
    // second parameter for values?

    let mut columns = String::new();
    for column_name in table.get_columns() {
        columns.push_str(&format!("{}, ", column_name));
    }

    // remove the trailing comma and space
    columns.pop();
    columns.pop();

    let sql = format!(
        "INSERT INTO {} (
            {}
        )
        VALUES (
            {}
        )",
        table.get_name(),
        columns,
        values
    );

    Ok(sql)
}
