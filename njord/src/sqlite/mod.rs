use std::env;

use rusqlite::{Connection, Error};

pub mod insert;
pub use insert::insert;
pub mod select;
pub use select::select;
use crate::util::find_target_directory;

pub mod query;

/// Open a database connection
pub fn open(db_name: &str) -> Result<Connection, Error> {
    let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "../target".to_string());
    let db_file_path = format!("{}/{}", target_dir, &db_name);
    let conn = Connection::open(db_file_path)?;

    Ok(conn)
}

/// Open an in-memory database connection
pub fn open_in_memory() -> Result<Connection, Error> {
    let conn = Connection::open_in_memory()?;

    Ok(conn)
}
