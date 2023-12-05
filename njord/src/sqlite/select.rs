use crate::sqlite::query::QueryBuilder;

use rusqlite::Connection;

pub fn select<'a>(conn: Connection, columns: Vec<String>) -> QueryBuilder<'a> {
    QueryBuilder::new(conn, columns)
}
