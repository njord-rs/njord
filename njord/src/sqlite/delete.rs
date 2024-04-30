use std::collections::HashMap;

use crate::condition::Condition;

use rusqlite::{Connection, Result};

use log::info;

use crate::table::Table;

pub fn delete<T: Table + Default>(conn: Connection) -> DeleteQueryBuilder<T> {
    DeleteQueryBuilder::new(conn)
}

pub struct DeleteQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
    where_condition: Option<Condition>,
    order_by: Option<HashMap<Vec<String>, String>>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<T: Table + Default> DeleteQueryBuilder<T> {
    pub fn new(conn: Connection) -> Self {
        DeleteQueryBuilder {
            conn,
            table: None,
            where_condition: None,
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    pub fn from(mut self, table: T) -> Self {
        self.table = Some(table);
        self
    }

    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.where_condition = Some(condition);
        self
    }

    pub fn order_by(mut self, col_and_order: HashMap<Vec<String>, String>) -> Self {
        self.order_by = Some(col_and_order);
        self
    }

    pub fn limit(mut self, count: usize) -> Self {
        self.limit = Some(count);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(self) {}
}
