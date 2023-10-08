use std::env;

use rusqlite::{Connection, Error};

pub mod init;
pub use init::init;
pub mod insert;
pub use insert::insert;
pub mod drop;
pub use drop::drop_table;

/// Open a database connection
pub fn open(db_name: &str) -> Result<Connection, Error> {
    let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "../target".to_string());
    let db_file_path = format!("{}/{}", target_dir, &db_name);
    let conn = Connection::open(db_file_path)?;

    Ok(conn)
}
