//! BSD 3-Clause License
//!
//! Copyright (c) 2024
//!     Marcus Cvjeticanin
//!     Chase Willden
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::{
    column::Column,
    condition::Condition,
    mysql::util::{
        generate_group_by_str, generate_having_str, generate_limit_str, generate_offset_str,
        generate_order_by_str, generate_where_condition_str,
    },
    query::QueryBuilder,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use log::info;
use mysql::prelude::*;
use mysql::{prelude::FromRow, Error, PooledConn};

use crate::table::Table;
use crate::util::{Join, JoinType};

/// Constructs a new SELECT query builder.
///
/// # Arguments
///
/// * `conn` - A `PooledConn` to the MySql database.
/// * `columns` - A vector of strings representing the columns to be selected.
///
/// # Returns
///
/// A `SelectQueryBuilder` instance.
pub fn select<'a, T: Table + Default + FromRow>(
    conn: &'a mut PooledConn,
    columns: Vec<Column<'a, T>>,
) -> SelectQueryBuilder<'a, T> {
    SelectQueryBuilder::new(conn, columns)
}

/// A builder for constructing SELECT queries.
#[derive(Clone)]
pub struct SelectQueryBuilder<'a, T: Table + Default + FromRow> {
    conn: Rc<RefCell<&'a mut PooledConn>>,
    table: Option<T>,
    columns: Vec<Column<'a, T>>,
    where_condition: Option<Condition<'a>>,
    distinct: bool,
    group_by: Option<Vec<String>>,
    order_by: Option<HashMap<Vec<String>, String>>,
    limit: Option<usize>,
    offset: Option<usize>,
    having_condition: Option<Condition<'a>>,
    except_clauses: Option<Vec<SelectQueryBuilder<'a, T>>>,
    union_clauses: Option<Vec<SelectQueryBuilder<'a, T>>>,
    joins: Option<Vec<Join<'a>>>,
}

impl<'a, T: Table + Default + FromRow> SelectQueryBuilder<'a, T> {
    /// Creates a new `SelectQueryBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `conn` - A `PooledConn` to the MySql database.
    /// * `columns` - A vector of strings representing the columns to be selected.
    pub fn new(conn: &'a mut PooledConn, columns: Vec<Column<'a, T>>) -> Self {
        SelectQueryBuilder {
            conn: Rc::new(RefCell::new(conn)),
            table: None,
            columns,
            where_condition: None,
            distinct: false,
            group_by: None,
            order_by: None,
            limit: None,
            offset: None,
            having_condition: None,
            except_clauses: None,
            union_clauses: None,
            joins: None,
        }
    }

    /// Sets the columns to be selected.
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of strings representing the columns to be selected.
    pub fn select(mut self, columns: Vec<Column<'a, T>>) -> Self {
        self.columns = columns;
        self
    }

    /// Sets the DISTINCT keyword for the query.
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Sets the table from which to select data.
    ///
    /// # Arguments
    ///
    /// * `table` - The table from which to select data.
    pub fn from(mut self, table: T) -> Self {
        self.table = Some(table);
        self
    }

    /// Sets the WHERE clause condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to be applied in the WHERE clause.
    pub fn where_clause(mut self, condition: Condition<'a>) -> Self {
        self.where_condition = Some(condition);
        self
    }

    /// Sets the GROUP BY clause columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A vector of strings representing the columns to be grouped by.
    pub fn group_by(mut self, columns: Vec<String>) -> Self {
        self.group_by = Some(columns);
        self
    }

    /// Sets the ORDER BY clause columns and order direction.
    ///
    /// # Arguments
    ///
    /// * `col_and_order` - A HashMap containing column names as keys and order direction as values.
    pub fn order_by(mut self, col_and_order: HashMap<Vec<String>, String>) -> Self {
        self.order_by = Some(col_and_order);
        self
    }

    /// Sets the LIMIT clause for the query.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of rows to limit the result to.
    pub fn limit(mut self, count: usize) -> Self {
        self.limit = Some(count);
        self
    }

    /// Sets the OFFSET clause for the query.
    ///
    /// # Arguments
    ///
    /// * `offset` - The number of rows to skip.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the HAVING clause condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to be applied in the HAVING clause.
    pub fn having(mut self, condition: Condition<'a>) -> Self {
        self.having_condition = Some(condition);
        self
    }

    /// Adds an EXCEPT clause to the query, allowing you to exclude results from another query.
    ///
    /// This method modifies the current query builder to exclude the results of the specified
    /// `other_query`. If there are already existing EXCEPT clauses, the new clause will be added
    /// to the list. If no EXCEPT clauses exist, a new list will be created with the provided
    /// query.
    ///
    /// # Arguments
    ///
    /// * `other_query` - A `SelectQueryBuilder` instance that represents the query whose results
    ///   should be excluded from the current query.
    ///
    /// # Returns
    ///
    /// Returns the modified `SelectQueryBuilder` instance with the new EXCEPT clause added.
    pub fn except(mut self, other_query: SelectQueryBuilder<'a, T>) -> Self {
        match self.except_clauses {
            Some(ref mut clauses) => clauses.push(other_query),
            None => self.except_clauses = Some(vec![other_query]),
        }
        self
    }

    /// Adds a UNION clause to the query, allowing you to combine results from another query.
    ///
    /// This method modifies the current query builder to include the results of the specified
    /// `other_query`. If there are already existing UNION clauses, the new clause will be added
    /// to the list. If no UNION clauses exist, a new list will be created with the provided
    /// query.
    ///
    /// # Arguments
    ///
    /// * `other_query` - A `SelectQueryBuilder` instance that represents the query whose results
    ///   should be combined with the current query.
    ///
    /// # Returns
    ///
    /// Returns the modified `SelectQueryBuilder` instance with the new UNION clause added.
    pub fn union(mut self, other_query: SelectQueryBuilder<'a, T>) -> Self {
        match self.union_clauses {
            Some(ref mut clauses) => clauses.push(other_query),
            None => self.union_clauses = Some(vec![other_query]),
        }
        self
    }

    /// Adds a JOIN clause to the query, allowing you to combine rows from two or more tables based on a related column.
    ///
    /// This method modifies the current query builder to include a join clause with the specified join type,
    /// target table, and condition for the join. If there are already existing JOIN clauses, the new clause
    /// will be added to the list. If no JOIN clauses exist, a new list will be created with the provided
    /// join information.
    ///
    /// # Arguments
    ///
    /// * `join_type` - The type of join to perform (e.g., INNER, LEFT, RIGHT, FULL).
    /// * `table` - The table to join with the current table.
    /// * `on_condition` - The condition that specifies how the tables are related (the ON clause).
    ///
    /// # Returns
    ///
    /// Returns the modified `SelectQueryBuilder` instance with the new JOIN clause added.
    pub fn join(
        mut self,
        join_type: JoinType,
        table: Arc<dyn Table>,
        on_condition: Condition<'a>,
    ) -> Self {
        match self.joins {
            Some(ref mut joins) => joins.push(Join::new(join_type, table, on_condition)),
            None => self.joins = Some(vec![Join::new(join_type, table, on_condition)]),
        }
        self
    }

    /// Builds the query string, this function should be used internally.
    pub fn build_query(&self) -> String {
        let columns_str = self
            .columns
            .iter()
            .map(|c| c.build())
            .collect::<Vec<String>>()
            .join(", ");

        let table_name = self
            .table
            .as_ref()
            .map(|t| t.get_name().to_string())
            .unwrap_or("".to_string());

        // Generate JOIN clauses, if any
        let join_clauses: Vec<String> = match &self.joins {
            Some(joins) => joins
                .iter()
                .map(|join| {
                    let join_type_str = match join.join_type {
                        JoinType::Inner => "INNER JOIN",
                        JoinType::Left => "LEFT JOIN",
                        JoinType::Right => "RIGHT JOIN",
                        JoinType::Full => "FULL OUTER JOIN",
                    };
                    format!(
                        "{} {} ON {}",
                        join_type_str,
                        join.table.get_name(),
                        generate_where_condition_str(Some(join.on_condition.clone()))
                            .replace("WHERE", "")
                    )
                })
                .collect(),
            None => Vec::new(),
        };

        let distinct_str = if self.distinct { "DISTINCT " } else { "" };
        let where_condition_str = generate_where_condition_str(self.where_condition.clone());
        let group_by_str = generate_group_by_str(&self.group_by);
        let order_by_str = generate_order_by_str(&self.order_by);
        let limit_str = generate_limit_str(self.limit);
        let offset_str = generate_offset_str(self.offset);
        let having_str =
            generate_having_str(self.group_by.is_some(), self.having_condition.as_ref());

        // Create the JOIN clause or an empty string
        let join_clause = if !join_clauses.is_empty() {
            join_clauses.join(" ")
        } else {
            String::new()
        };

        let mut query = format!(
            "SELECT {}{} FROM {} {} {} {} {} {} {}",
            distinct_str,
            columns_str,
            table_name,
            join_clause,
            where_condition_str,
            group_by_str,
            having_str,
            order_by_str,
            format!("{} {}", limit_str, offset_str),
        );

        // Handle EXCEPT clauses
        if let Some(except_clauses) = &self.except_clauses {
            for except_query in except_clauses {
                let except_sql = except_query.build_query();
                query = format!("{} EXCEPT {}", query, except_sql);
            }
        }

        // Handle UNION clauses
        if let Some(union_clauses) = &self.union_clauses {
            for union_query in union_clauses {
                let union_sql = union_query.build_query();
                query = format!("{} UNION {}", query, union_sql);
            }
        }

        query
    }

    /// Builds and executes the SELECT query.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of selected table rows if successful,
    /// or a `rusqlite::Error` if an error occurs during the execution.
    pub fn build(&mut self) -> Result<Vec<T>, Error> {
        let final_query = self.build_query();

        info!("{}", final_query);
        println!("{}", final_query);

        // Borrow the connection mutably from the RefCell
        let mut conn = self.conn.borrow_mut();

        let results: Vec<T> = conn.query(final_query.as_str())?;

        Ok(results)
    }
}

/// Implement `QueryBuilder` for `SelectQueryBuilder`
///
/// The where statement ensures the T is long lived
impl<'a, T> QueryBuilder<'a> for SelectQueryBuilder<'a, T>
where
    T: Table + Default + Clone + FromRow + 'a, // Added 'a bound here
{
    fn to_sql(&self) -> String {
        self.build_query()
    }
}
