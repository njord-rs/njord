pub fn update(table: &dyn Table) -> Result<()> {
    let mut conn = open()?;

    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement(&**table);
    tx.execute(&statement, [])?;

    // commit the transaction
    tx.commit()?;

    info!("Update operation, done.");

    Ok(())
}
