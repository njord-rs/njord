use crate::table::Table;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

/// drop a table
pub fn drop_table<T: Table>(mut conn: Connection) -> Result<()> {
    // create a transaction
    let tx = conn.transaction()?;

    let statement = generate_statement();

    let generated_statement = match statement {
        Ok(statement) => statement,
        Err(error) => panic!("Problem generating statement: {:?}.", error),
    };

    tx.execute(generated_statement.as_str(), [])?;

    // commit the transaction
    tx.commit()?;

    info!("Drop table, done.");

    Ok(())
}

fn generate_statement<T: Table>() -> Result<String, Error> {
    let sql = format!("DROP TABLE {};", T::get_name());

    println!("{}", sql);

    Ok(sql)
}
