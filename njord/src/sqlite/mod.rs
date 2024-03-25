use std::path::Path;

use rusqlite::{Connection, Error};

pub mod insert;
pub use insert::insert;
pub mod select;
pub use select::select;

pub mod query;

/// Open a database connection
pub fn open(db_path: &Path) -> Result<Connection, Error> {
    let conn = Connection::open(db_path)?;

    Ok(conn)
}

/// Open an in-memory database connection
pub fn open_in_memory() -> Result<Connection, Error> {
    let conn = Connection::open_in_memory()?;

    Ok(conn)
}
