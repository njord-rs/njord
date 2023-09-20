use std::env;

use rusqlite::{Connection, Error};

pub mod init;
pub mod insert;

/// open database connection
pub fn open() -> Result<Connection, Error> {
    let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "../target".to_string());
    let db_file_name = "my_database.db";
    let db_file_path = format!("{}/{}", target_dir, db_file_name);
    let conn = Connection::open(&db_file_path)?;

    Ok(conn)
}
