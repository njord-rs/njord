use crate::{sqlite::query::QueryBuilder, table::Table};

use rusqlite::Connection;

pub fn select<T: Table + Default>(conn: Connection, columns: Vec<String>) -> QueryBuilder<T> {
    QueryBuilder::new(conn, columns)
}
