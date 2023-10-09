use crate::table::Table;

use rusqlite::{Connection, Result};

use super::Condition;

pub struct QueryBuilder<'a> {
    conn: Connection,
    table: Option<&'a dyn Table>,
    columns: Vec<String>,
    condition: Option<Condition>,
    selected: bool,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(conn: Connection, columns: Vec<String>) -> Self {
        QueryBuilder {
            conn,
            table: None,
            columns,
            condition: None,
            selected: false,
        }
    }

    pub fn select(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self.selected = true;
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

    pub fn build(self) -> Result<()> {
        let columns_str = self.columns.join(", ");
        println!("SELECT {} FROM", columns_str);
        if let Some(condition) = self.condition {
            match condition {
                Condition::Eq(column, value) => println!("WHERE {} = '{}'", column, value),
                Condition::Ne(column, value) => println!("WHERE {} <> '{}'", column, value),
                Condition::Lt(column, value) => println!("WHERE {} < '{}'", column, value),
                Condition::Gt(column, value) => println!("WHERE {} > '{}'", column, value),
                Condition::Le(column, value) => println!("WHERE {} <= '{}'", column, value),
                Condition::Ge(column, value) => println!("WHERE {} >= '{}'", column, value),
                Condition::And(left, right) => {
                    println!("WHERE ({}) AND ({})", left.build(), right.build())
                }
                Condition::Or(left, right) => {
                    println!("WHERE ({}) OR ({})", left.build(), right.build())
                }
            }
        }
        Ok(())
    }
}
