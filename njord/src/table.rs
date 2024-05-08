//! BSD 3-Clause License
//!
//! Copyright (c) 2024, Marcus Cvjeticanin
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

use std::collections::HashMap;

#[allow(unused_imports)]
use njord_derive::Table;

/// The Table trait.
///
/// It is used for structs that want need the behaviour of an SQL Table.
pub trait Table {
    /// Get the name of the table.
    ///
    /// Returns a reference to a string representing the name of the table.
    fn get_name(&self) -> &str;

    /// Get the columns of the table.
    ///
    /// Returns a reference to a `HashMap` where the keys are column names,
    /// and the values are column types represented as strings.
    fn get_columns(&self) -> HashMap<String, String>;

    /// Get the names of the columns.
    ///
    /// Returns a `Vec<String>` containing the names of the columns in the same order
    /// as they appear in the table.
    fn get_column_fields(&self) -> Vec<String>;

    /// Get the values of the columns.
    ///
    /// Returns a `Vec<String>` containing the values of the columns in the same order
    /// as they appear in the table.
    fn get_column_values(&self) -> Vec<String>;

    /// Set the values of the columns.
    fn set_column_value(&mut self, column: &str, value: &str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tables() {
        #[derive(Table)]
        #[table_name = "table_a"]
        struct TableA {
            title: String,
            desc: String,
            amount: u32,
        }

        let table_a = TableA {
            title: "Table A".to_string(),
            desc: "Some description for Table A".to_string(),
            amount: 0,
        };

        #[derive(Table)]
        #[table_name = "table_b"]
        struct TableB {
            name: String,
            age: u32,
            email: String,
        }

        let table_b = TableB {
            name: "John Doe".to_string(),
            age: 30,
            email: "john.doe@example.com".to_string(),
        };

        #[derive(Table)]
        #[table_name = "table_c"]
        struct TableC {
            product_id: i64,
            product_name: String,
            price: f64,
            in_stock: bool,
        }

        let table_c = TableC {
            product_id: 1001,
            product_name: "Example Product".to_string(),
            price: 29.99,
            in_stock: true,
        };

        let expected_columns_a: HashMap<String, String> = vec![
            ("title".to_string(), "TEXT".to_string()),
            ("desc".to_string(), "TEXT".to_string()),
            ("amount".to_string(), "INTEGER".to_string()),
        ]
        .into_iter()
        .collect();

        let expected_columns_b: HashMap<String, String> = vec![
            ("name".to_string(), "TEXT".to_string()),
            ("age".to_string(), "INTEGER".to_string()),
            ("email".to_string(), "TEXT".to_string()),
        ]
        .into_iter()
        .collect();

        let expected_columns_c: HashMap<String, String> = vec![
            ("product_id".to_string(), "INTEGER".to_string()),
            ("product_name".to_string(), "TEXT".to_string()),
            ("price".to_string(), "REAL".to_string()),
            ("in_stock".to_string(), "TEXT".to_string()),
        ]
        .into_iter()
        .collect();

        // setting up the expected fields
        let expected_fields_a: Vec<String> = vec![
            "title".to_string(),
            "desc".to_string(),
            "amount".to_string(),
        ];

        let expected_fields_b: Vec<String> =
            vec!["name".to_string(), "age".to_string(), "email".to_string()];

        let expected_fields_c: Vec<String> = vec![
            "product_id".to_string(),
            "product_name".to_string(),
            "price".to_string(),
            "in_stock".to_string(),
        ];

        // assert that we get the expected column names
        let columns_a = table_a.get_columns();
        for (key, value) in &expected_columns_a {
            assert_eq!(columns_a.get(key), Some(value));
        }

        let columns_b = table_b.get_columns();
        for (key, value) in &expected_columns_b {
            assert_eq!(columns_b.get(key), Some(value));
        }

        let columns_c = table_c.get_columns();
        for (key, value) in &expected_columns_c {
            assert_eq!(columns_c.get(key), Some(value));
        }

        // assert that we get the correct table fields
        assert_eq!(table_a.get_column_fields(), expected_fields_a);
        assert_eq!(table_b.get_column_fields(), expected_fields_b);
        assert_eq!(table_c.get_column_fields(), expected_fields_c);

        // assert that we get the correct table name
        assert_eq!(table_a.get_name(), "table_a");
        assert_eq!(table_b.get_name(), "table_b");
        assert_eq!(table_c.get_name(), "table_c");
    }
}
