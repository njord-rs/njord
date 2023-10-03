use std::collections::HashMap;

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
    fn get_columns(&self) -> &HashMap<String, String>;

    /// Get the names of the columns.
    ///
    /// Returns a `Vec<String>` containing the names of the columns in the same order
    /// as they appear in the table.
    fn get_column_fields(&self) -> Vec<String>;
}
