use std::collections::HashMap;

/// The Table trait.
///
/// It is used for structs that want need the behaviour of an SQL Table.
pub trait Table {
    fn get_name(&self) -> &str;
    fn get_columns(&self) -> &HashMap<String, String>;
    fn get_column_fields(&self) -> Vec<String>;
}

/// The TableDefinition struct.
///
/// It is used for implementing Table trait.
pub struct TableDefinition {
    pub name: String,
    pub columns: HashMap<String, String>,
}
