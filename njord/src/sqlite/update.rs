use crate::condition::Condition;
use std::collections::HashMap;

use rusqlite::{Connection, Result};

use log::info;
use rusqlite::types::Value;

use crate::table::Table;

pub fn update<T: Table + Default>(conn: Connection, table: T) -> UpdateQueryBuilder<T> {
    UpdateQueryBuilder::new(conn, table)
}

pub struct UpdateQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
    // set: T,
    where_condition: Option<Condition>,
}

impl<T: Table + Default> UpdateQueryBuilder<T> {
    pub fn new(conn: Connection, table: T) -> Self {
        UpdateQueryBuilder {
            conn,
            table: Some(table),
            // set: None,
            where_condition: None,
        }
    }

    pub fn set(mut self, table: T) -> Self {
        // self.set = table;
        self
    }

    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.where_condition = Some(condition);
        self
    }

    pub fn build(self) -> Result<(), String> {
        Ok(())
    }
}
