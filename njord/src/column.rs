use crate::{sqlite::select::SelectQueryBuilder, table::Table};

/// Define the enum to represent a column as either a String or SelectQueryBuilder
#[derive(Clone)]
pub enum Column<'a, T: Table + Default> {
    Text(String),
    SubQuery(SelectQueryBuilder<'a, T>),
}

// Implement the build method to convert the enum to a string
impl<'a, T: Table + Default> Column<'a, T> {
    /// Helper function to convert the columns to a string
    pub fn build(&self) -> String {
        match self {
            Column::Text(text) => text.clone(),
            Column::SubQuery(sub_query) => "(".to_string() + &sub_query.build_query() + ")",
        }
    }
}

// Implementation of fmt::Display for Column
impl<'a, T: Table + Default> std::fmt::Display for Column<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

// Implementation of PartialEq for Column
impl<'a, T: Table + Default> PartialEq for Column<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.build() == other.build()
    }
}

// Implementation of PartialEq<String> for Column
impl<'a, T: Table + Default> PartialEq<String> for Column<'a, T> {
    fn eq(&self, other: &String) -> bool {
        match self {
            Column::Text(text) => text == other,
            Column::SubQuery(sub_query) => sub_query.build_query() == *other,
        }
    }
}

// Implementation of PartialEq<&str> for Column
impl<'a, T: Table + Default> PartialEq<&str> for Column<'a, T> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Column::Text(text) => text == other,
            Column::SubQuery(sub_query) => sub_query.build_query() == *other,
        }
    }
}
