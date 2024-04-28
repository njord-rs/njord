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
    set: Option<T>,
    where_condition: Option<Condition>,
}

impl<T: Table + Default> UpdateQueryBuilder<T> {
    pub fn new(conn: Connection, table: T) -> Self {
        UpdateQueryBuilder {
            conn,
            table: Some(table),
            set: None,
            where_condition: None,
        }
    }

    pub fn set(mut self, table: T) -> Self {
        self.set = Some(table);
        self
    }

    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.where_condition = Some(condition);
        self
    }

    pub fn build(self) -> Result<(), String> {
        let table_name = self
            .table
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // sanitize table name from unwanted quotations or backslashes
        let table_name_str = table_name.replace("\"", "").replace("\\", "");

        // generate = SET column1 = value1, column2 = value2, ...
        let set = "";

        let where_condition_str = if let Some(condition) = self.where_condition {
            format!("WHERE {}", condition.build())
        } else {
            String::new()
        };

        // construct the query based on defined variables above
        let query = format!(
            "UPDATE {} SET {} {}",
            table_name_str, set, where_condition_str
        );

        info!("{}", query);
        println!("{}", query);

        // prepare sql statement
        match self.conn.prepare(query.as_str()) {
            Ok(_) => println!("Success!"),
            Err(_) => eprintln!("Could not execute..."),
        };

        Ok(())
    }
}
