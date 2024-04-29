use std::collections::HashMap;

use crate::condition::Condition;

use rusqlite::{Connection, Result};

use log::info;

use crate::table::Table;

pub fn delete<T: Table + Default>(conn: Connection, table: T) -> DeleteQueryBuilder<T> {
    DeleteQueryBuilder::new(conn, table)
}

pub struct DeleteQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
}

impl<T: Table + Default> DeleteQueryBuilder<T> {
    pub fn new(conn: Connection, table: T) -> Self {
        DeleteQueryBuilder {
            conn,
            table: Some(table),
        }
    }
}
