use crate::sqlite::query::QueryBuilder;

use rusqlite::Connection;

pub fn select<'a>(conn: Connection, columns: Vec<String>) -> QueryBuilder<'a> {
    // here we need to get the SQL string from QueryBuilder and then use that to run the SQL
    // then map the result against our Table trait and return a vector of structs
    // so we can iterate over them.
    QueryBuilder::new(conn, columns)
}
