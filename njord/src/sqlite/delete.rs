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

    pub fn build(self) -> Result<(), String> {
        let table_name = self
            .table
            .as_ref()
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = table_name.replace("\"", "").replace("\\", "");

        let where_condition_str = if let Some(condition) = self.where_condition {
            format!("WHERE {}", condition.build())
        } else {
            String::new()
        };

        let order_by_str = if let Some(order_by) = &self.order_by {
            let order_by_str: Vec<String> = order_by
                .iter()
                .map(|(columns, order)| format!("{} {}", columns.join(", "), order))
                .collect();
            if !order_by_str.is_empty() {
                format!("ORDER BY {}", order_by_str.join(", "))
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let limit_str = self
            .limit
            .map_or(String::new(), |count| format!("LIMIT {}", count));
        let offset_str = self
            .offset
            .map_or(String::new(), |offset| format!("OFFSET {}", offset));

        // Construct the query based on defined variables above
        let query = format!(
            "UPDATE {} SET {} {} {}",
            table_name_str,
            where_condition_str,
            order_by_str,
            format!("{} {}", limit_str, offset_str),
        );

        info!("{}", query);
        println!("{}", query);

        // Prepare SQL statement
        match self.conn.prepare(query.as_str()) {
            Ok(_) => println!("Success!"),
            Err(_) => eprintln!("Could not execute..."),
        };

        Ok(())
    }
}
