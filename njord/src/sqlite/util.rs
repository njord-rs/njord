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

use crate::condition::Condition;

/// Generates an SQL WHERE clause string based on the provided condition.
///
/// If `condition` is Some, it constructs an SQL WHERE clause string with the specified condition.
/// If `condition` is None, an empty string is returned.
///
/// # Arguments
///
/// * `condition` - An Option containing the condition.
///
/// # Returns
///
/// A String representing the generated SQL WHERE clause.
pub fn generate_where_condition_str(condition: Option<Condition>) -> String {
    if let Some(condition) = condition {
        format!("WHERE {}", condition.build())
    } else {
        String::new()
    }
}

/// Generates an SQL GROUP BY clause string based on the provided columns.
///
/// If `columns` is Some, it constructs an SQL GROUP BY clause string with the specified columns.
/// If `columns` is None, an empty string is returned.
///
/// # Arguments
///
/// * `columns` - An Option containing a reference to a vector of column names.
///
/// # Returns
///
/// A String representing the generated SQL GROUP BY clause.
pub fn generate_group_by_str(columns: &Option<Vec<String>>) -> String {
    match columns {
        Some(columns) => format!("GROUP BY {}", columns.join(", ")),
        None => String::new(),
    }
}

/// Generates an SQL ORDER BY clause string based on the provided `order_by` option.
///
/// If `order_by` is Some, it should contain a HashMap where the keys are vectors of column names
/// and the values are corresponding sort orders (ASC or DESC). This function constructs an SQL
/// ORDER BY clause string based on the content of the HashMap. If the HashMap is empty, an empty
/// string is returned.
///
/// # Arguments
///
/// * `order_by` - An Option containing a HashMap where the keys are vectors of column names and
///   the values are corresponding sort orders (ASC or DESC).
///
/// # Returns
///
/// A String representing the generated SQL ORDER BY clause.
pub fn generate_order_by_str(order_by: &Option<HashMap<Vec<String>, String>>) -> String {
    let order_by_str = if let Some(order_by) = order_by.as_ref() {
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

    return order_by_str;
}

/// Generates an SQL LIMIT clause string based on the provided limit count.
///
/// If `limit` is Some, it constructs an SQL LIMIT clause string with the specified count.
/// If `limit` is None, an empty string is returned.
///
/// # Arguments
///
/// * `limit` - An Option containing the limit count.
///
/// # Returns
///
/// A String representing the generated SQL LIMIT clause.
pub fn generate_limit_str(limit: Option<usize>) -> String {
    limit.map_or(String::new(), |count| format!("LIMIT {}", count))
}

/// Generates an SQL OFFSET clause string based on the provided offset count.
///
/// If `offset` is Some, it constructs an SQL OFFSET clause string with the specified count.
/// If `offset` is None, an empty string is returned.
///
/// # Arguments
///
/// * `offset` - An Option containing the offset count.
///
/// # Returns
///
/// A String representing the generated SQL OFFSET clause.
pub fn generate_offset_str(offset: Option<usize>) -> String {
    offset.map_or(String::new(), |offset| format!("OFFSET {}", offset))
}

/// Generates an SQL HAVING clause string based on the provided group by flag and condition.
///
/// If `group_by` is true and `having_condition` is Some, it constructs an SQL HAVING clause string
/// with the specified condition.
/// If either `group_by` is false or `having_condition` is None, an empty string is returned.
///
/// # Arguments
///
/// * `group_by` - An Option indicating whether GROUP BY is present.
/// * `having_condition` - An Option containing the condition for the HAVING clause.
///
/// # Returns
///
/// A String representing the generated SQL HAVING clause.
pub fn generate_having_str(group_by: bool, having_condition: Option<&Condition>) -> String {
    if group_by && having_condition.is_some() {
        format!("HAVING {}", having_condition.unwrap().build())
    } else {
        String::new()
    }
}

/// Removes double quotes and backslashes from a given string.
///
/// # Arguments
///
/// * `input` - The input string from which double quotes and backslashes will be removed.
///
/// # Returns
///
/// A String with double quotes and backslashes removed.
pub fn remove_quotes_and_backslashes(input: &str) -> String {
    input.replace("\"", "").replace("\\", "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition::Condition;

    #[test]
    fn test_generate_where_condition_str() {
        // Test when condition is Some
        let condition = Condition::Eq("age".to_string(), "30".to_string());
        let result = generate_where_condition_str(Some(condition)); // TODO: need to fix this later
                                                                    // assert_eq!(result, format!("WHERE {}", condition.build()));

        // Test when condition is None
        let result = generate_where_condition_str(None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_group_by_str() {
        // Test when columns is Some
        let columns = Some(vec!["name".to_string(), "age".to_string()]);
        let result = generate_group_by_str(&columns);
        assert_eq!(result, "GROUP BY name, age");

        // Test when columns is None
        let result = generate_group_by_str(&None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_order_by_str() {
        // Test when order_by is Some
        let mut map = HashMap::new();
        map.insert(vec!["name".to_string()], "ASC".to_string());
        map.insert(vec!["age".to_string()], "DESC".to_string());
        let result = generate_order_by_str(&Some(map));
        assert_eq!(result, "ORDER BY name ASC, age DESC");

        // Test when order_by is None
        let result = generate_order_by_str(&None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_limit_str() {
        // Test when limit is Some
        let result = generate_limit_str(Some(10));
        assert_eq!(result, "LIMIT 10");

        // Test when limit is None
        let result = generate_limit_str(None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_offset_str() {
        // Test when offset is Some
        let result = generate_offset_str(Some(5));
        assert_eq!(result, "OFFSET 5");

        // Test when offset is None
        let result = generate_offset_str(None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_having_str() {
        // Test when group_by is true and having_condition is Some
        let condition = Condition::Gt("COUNT(age)".to_string(), "5".to_string());
        let result = generate_having_str(true, Some(&condition));
        assert_eq!(result, format!("HAVING {}", condition.build()));

        // Test when group_by is false
        let result = generate_having_str(false, Some(&condition));
        assert_eq!(result, "");

        // Test when having_condition is None
        let result = generate_having_str(true, None);
        assert_eq!(result, "");

        // Test when both group_by is false and having_condition is None
        let result = generate_having_str(false, None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_remove_quotes_and_backslashes() {
        let input = r#""table_name\"""#;
        let result = remove_quotes_and_backslashes(input);
        assert_eq!(result, "table_name");
    }
}
