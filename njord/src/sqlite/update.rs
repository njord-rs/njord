use crate::condition::Condition;
use std::collections::HashMap;

use rusqlite::{Connection, Result};

use log::info;
use rusqlite::types::Value;

use crate::table::Table;

pub fn update<T: Table + Default>(conn: Connection, columns: Vec<String>) -> UpdateQueryBuilder<T> {
    UpdateQueryBuilder::new(conn, columns)
}

pub struct UpdateQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
}

impl<T: Table + Default> UpdateQueryBuilder<T> {
    pub fn new(conn: Connection, columns: Vec<String>) -> Self {
        UpdateQueryBuilder { conn, table: None }
    }
}
