pub fn select(table: &dyn Table) -> Result<Vec<Vec<String>>, Error> {
    let mut conn = open()?;

    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement(&**table);
    let mut rows = Vec::new();

    let mut stmt = tx.prepare(&statement)?;
    let mut result_set = stmt.query([])?;

    // iterate over the result set and collect data into rows
    while let Some(row) = result_set.next()? {
        let row_values: Vec<String> = row.iter().map(|col| col.to_string()).collect();
        rows.push(row_values);
    }

    // commit the transaction
    tx.commit()?;

    info!("Select operation, done.");

    Ok(rows)
}
