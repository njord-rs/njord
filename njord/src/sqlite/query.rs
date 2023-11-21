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
    group_by: Option<Vec<String>>,
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
            group_by: None,
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

    pub fn group_by(mut self, columns: Vec<String>) -> Self {
        self.group_by = Some(columns);
        self
    }

    pub fn build(self) -> Result<Vec<HashMap<String, Value>>> {
        let columns_str = self.columns.join(", ");
        let table_name = self
            .table
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        let distinct_str = if self.distinct { "DISTINCT " } else { "" };
        let group_by_str = match &self.group_by {
            Some(columns) => format!("GROUP BY {}", columns.join(", ")),
            None => String::new(),
        };

        let mut query = format!(
            "SELECT {}{} FROM {}",
            distinct_str,
            columns_str,
            table_name
        );

        println!("{}", query);

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

        // GROUP BY must be placed after the WHERE clause
        if !group_by_str.is_empty() {
            query.push_str(&group_by_str);
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
