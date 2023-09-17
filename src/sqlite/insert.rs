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

// might need to re-think this one
/* fn generate_statement(table: &dyn Table, values: Vec<String>) -> Result<String, Error> {
    // second parameter for values?

    let mut columns_str = String::new();
    for column_name in table.get_column_names() {
        columns_str.push_str(&format!("{}, ", column_name));
    }

    let mut values_str = String::new();
    for value in values {
        values_str.push_str(&format!("{}, ", value))
    }

    // remove the trailing comma and space
    columns_str.pop();
    columns_str.pop();
    values_str.pop();
    values_str.pop();

    let sql = format!(
        "INSERT INTO {} (
            {}
        )
        VALUES (
            {}
        )",
        table.get_name(),
        columns_str,
        values_str
    );

    Ok(sql)
}

#[test]
fn test_insert() {
    let table1_values = vec!["New ORM library for Rust"];
    let table2_values = vec!["Rust is a great language!"];
} */
