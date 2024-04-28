use crate::condition::Condition;

use rusqlite::{Connection, Result};

use log::info;

use crate::table::Table;

pub fn update<T: Table + Default>(conn: Connection, table: T) -> UpdateQueryBuilder<T> {
    UpdateQueryBuilder::new(conn, table)
}

pub struct UpdateQueryBuilder<T: Table + Default> {
    conn: Connection,
    table: Option<T>,
    columns: Vec<String>,
    set: Option<T>,
    where_condition: Option<Condition>,
}

impl<T: Table + Default> UpdateQueryBuilder<T> {
    pub fn new(conn: Connection, table: T) -> Self {
        UpdateQueryBuilder {
            conn,
            table: Some(table),
            columns: Vec::new(),
            set: None,
            where_condition: None,
        }
    }

    pub fn set(mut self, columns: Vec<String>, table: T) -> Self {
        self.columns = columns;
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
            .as_ref()
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = table_name.replace("\"", "").replace("\\", "");

        // Generate SET clause
        let set = if let Some(table) = &self.table {
            let mut set_fields = Vec::new();
            let values = table.get_column_values();

            for column in &self.columns {
                // Check if column exists in the table's fields
                let value = match table.get_column_values().iter().position(|c| c == column) {
                    Some(index) => values.get(index).cloned().unwrap_or_default(),
                    None => {
                        // Handle the case when the column doesn't exist in the table
                        return Err(format!("Column '{}' does not exist in the table", column));
                    }
                };
                set_fields.push(format!("{} = {}", column, value));
            }

            set_fields.join(", ")
        } else {
            String::new()
        };

        let where_condition_str = if let Some(condition) = self.where_condition {
            format!("WHERE {}", condition.build())
        } else {
            String::new()
        };

        // Construct the query based on defined variables above
        let query = format!(
            "UPDATE {} SET {} {}",
            table_name_str, set, where_condition_str
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
