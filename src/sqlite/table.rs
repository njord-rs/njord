use std::collections::HashMap;

pub trait Table {
    fn get_name(&self) -> &str;
    fn get_columns(&self) -> &HashMap<String, String>;
    fn get_column_fields(&self) -> Vec<String>;
}

pub struct TableDefinition {
    pub name: String,
    pub columns: HashMap<String, String>,
}
