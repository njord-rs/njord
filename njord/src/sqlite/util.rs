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
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use crate::order_by_str;
///
/// let mut map = HashMap::new();
/// map.insert(vec!["name".to_string()], "ASC".to_string());
/// map.insert(vec!["age".to_string()], "DESC".to_string());
///
/// assert_eq!(order_by_str(&Some(map)), "ORDER BY name ASC, age DESC");
/// assert_eq!(order_by_str(&None), "");
/// ```
pub fn order_by_str(order_by: &Option<HashMap<Vec<String>, String>>) -> String {
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
