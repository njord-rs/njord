use crate::table::Table;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

/// drop a table
pub fn drop(mut conn: Connection, table: &dyn Table) -> Result<()> {
    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement(table);

    // let generated_statement = match statement {
    //     Ok(statement) => statement,
    //     Err(error) => panic!("Problem generating statement: {:?}.", error),
    // };

    tx.execute(&statement.unwrap(), [])?;

    // commit the transaction
    tx.commit()?;

    info!("Drop table, done.");

    Ok(())
}

fn generate_statement(table: &dyn Table) -> Result<String, Error> {
    let sql = format!("DROP TABLE {};", table.get_name());

    println!("SQL: {}", sql);

    Ok(sql)
}
