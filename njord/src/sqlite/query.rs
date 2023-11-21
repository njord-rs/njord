use std::collections::HashMap;
use crate::table::Table;

use rusqlite::{Connection, Result};

use log::info;
use rusqlite::types::Value;

use super::Condition;

pub struct QueryBuilder<'a> {
    conn: Connection,
    table: Option<&'a dyn Table>,
    columns: Vec<String>,
    condition: Option<Condition>,
    selected: bool,
    distinct: bool,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(conn: Connection, columns: Vec<String>) -> Self {
        QueryBuilder {
            conn,
            table: None,
            columns,
            condition: None,
            selected: false,
            distinct: false,
        }
    }

    pub fn select(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self.selected = true;
        self
    }

    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    pub fn from(mut self, table: &'a dyn Table) -> Self {
        self.table = Some(table);
        self
    }

    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn build(self) -> Result<Vec<HashMap<String, Value>>> {
        let columns_str = self.columns.join(", ");
        let table_name = self
            .table
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        let mut query;
        if self.distinct {
            query = format!("SELECT DISTINCT {} FROM {}", columns_str, table_name);
        } else {
            query = format!("SELECT {} FROM {}", columns_str, table_name);
        }

        if let Some(condition) = self.condition {
            match condition {
                Condition::Eq(column, value) => {
                    query.push_str(&format!(" WHERE {} = '{}'", column, value));
                }
                Condition::Ne(column, value) => {
                    query.push_str(&format!(" WHERE {} <> '{}'", column, value));
                }
                Condition::Lt(column, value) => {
                    query.push_str(&format!(" WHERE {} < '{}'", column, value));
                }
                Condition::Gt(column, value) => {
                    query.push_str(&format!(" WHERE {} > '{}'", column, value));
                }
                Condition::Le(column, value) => {
                    query.push_str(&format!(" WHERE {} <= '{}'", column, value));
                }
                Condition::Ge(column, value) => {
                    query.push_str(&format!(" WHERE {} >= '{}'", column, value));
                }
                Condition::And(left, right) => {
                    let left_query = left.build();
                    let right_query = right.build();
                    query.push_str(&format!(" WHERE ({}) AND ({})", left_query, right_query));
                }
                Condition::Or(left, right) => {
                    let left_query = left.build();
                    let right_query = right.build();
                    query.push_str(&format!(" WHERE ({}) OR ({})", left_query, right_query));
                }
            }
        }

        info!("{}", query);
        println!("{}", query);

        let mut stmt = self.conn.prepare(query.as_str())?;

        let iter = stmt.query_map((), |row| {
            let mut result_row = HashMap::new();
            for (i, column) in self.columns.iter().enumerate() {
                let value = row.get_unwrap::<usize, Value>(i);
                result_row.insert(column.clone(), value);
            }
            Ok(result_row)
        })?;

        let result: Result<Vec<HashMap<String, Value>>> = iter.collect();
        result.map_err(|err| err.into())
    }
}
