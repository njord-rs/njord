use crate::condition::Condition;
use std::collections::HashMap;

use rusqlite::{Connection, Result};

use log::info;
use rusqlite::types::Value;

use crate::table::Table;

pub fn select<T: Table + Default>(conn: Connection, columns: Vec<String>) -> SelectQueryBuilder<T> {
    SelectQueryBuilder::new(conn, columns)
}

pub struct SelectQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
    columns: Vec<String>,
    where_condition: Option<Condition>,
    selected: bool,
    distinct: bool,
    group_by: Option<Vec<String>>,
    order_by: Option<HashMap<Vec<String>, String>>,
    limit: Option<usize>,
    offset: Option<usize>,
    having_condition: Option<Condition>,
}

impl<T: Table + Default> SelectQueryBuilder<T> {
    pub fn new(conn: Connection, columns: Vec<String>) -> Self {
        SelectQueryBuilder {
            conn,
            table: None,
            columns,
            where_condition: None,
            selected: false,
            distinct: false,
            group_by: None,
            order_by: None,
            limit: None,
            offset: None,
            having_condition: None,
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

    pub fn from(mut self, table: T) -> Self {
        self.table = Some(table);
        self
    }

    pub fn where_clause(mut self, condition: Condition) -> Self {
        self.where_condition = Some(condition);
        self
    }

    pub fn group_by(mut self, columns: Vec<String>) -> Self {
        self.group_by = Some(columns);
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

    pub fn having(mut self, condition: Condition) -> Self {
        self.having_condition = Some(condition);
        self
    }

    pub fn build(self) -> Result<Vec<T>> {
        let columns_str = self.columns.join(", ");

        let table_name = self
            .table
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = table_name.replace("\"", "").replace("\\", "");

        let distinct_str = if self.distinct { "DISTINCT " } else { "" };

        let where_condition_str = if let Some(condition) = self.where_condition {
            format!("WHERE {}", condition.build())
        } else {
            String::new()
        };

        let group_by_str = match &self.group_by {
            Some(columns) => format!("GROUP BY {}", columns.join(", ")),
            None => String::new(),
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

        // Having should only be added if group_by is present
        let having_str = if self.group_by.is_some() && self.having_condition.is_some() {
            format!("HAVING {}", self.having_condition.unwrap().build())
        } else {
            String::new()
        };

        // Construct the query based on defined variables above
        let query = format!(
            "SELECT {}{} FROM {} {} {} {} {} {}",
            distinct_str,
            columns_str,
            table_name_str,
            where_condition_str,
            group_by_str,
            having_str,
            order_by_str,
            format!("{} {}", limit_str, offset_str),
        );

        info!("{}", query);
        println!("{}", query);

        // Prepare SQL statement
        let mut stmt = self.conn.prepare(query.as_str())?;

        let iter = stmt.query_map((), |row| {
            // Dynamically create an instance of the struct based on the Table trait
            let mut instance = T::default();
            let columns = instance.get_column_fields();

            for (index, column) in columns.iter().enumerate() {
                // Use the index to get the value from the row and set it in the struct
                let value = row.get::<usize, Value>(index)?;

                let string_value = match value {
                    Value::Integer(val) => val.to_string(),
                    Value::Null => String::new(),
                    Value::Real(val) => val.to_string(),
                    Value::Text(val) => val.to_string(),
                    Value::Blob(val) => String::from_utf8_lossy(&val).to_string(),
                };

                instance.set_column_value(column, &string_value);
            }

            Ok(instance)
        })?;

        let result: Result<Vec<T>> = iter
            .map(|row_result| row_result.and_then(|row| Ok(row)))
            .collect::<Result<Vec<T>>>();

        result.map_err(|err| err.into())
    }
}
