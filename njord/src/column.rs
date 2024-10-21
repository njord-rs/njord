use crate::query::QueryBuilder;

/// Define the enum to represent a column as either a String or SelectQueryBuilder
#[derive(Clone)]
pub enum Column<'a> {
    Text(String),
    // Subquery with alias
    SubQuery(Box<dyn QueryBuilder<'a> + 'a>, String),
}

// Implement the build method to convert the enum to a string
impl<'a> Column<'a> {
    /// Helper function to convert the columns to a string
    pub fn build(&self) -> String {
        match self {
            Column::Text(text) => text.clone(),
            Column::SubQuery(sub_query, alias) => {
                "(".to_string() + &sub_query.to_sql() + ") AS " + alias
            }
        }
    }
}

// Implementation of fmt::Display for Column
impl<'a> std::fmt::Display for Column<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

// Implementation of PartialEq for Column
impl<'a> PartialEq for Column<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.build() == other.build()
    }
}

// Implementation of PartialEq<String> for Column
impl<'a> PartialEq<String> for Column<'a> {
    fn eq(&self, other: &String) -> bool {
        match self {
            Column::Text(text) => text == other,
            Column::SubQuery(sub_query, _) => sub_query.to_sql() == *other,
        }
    }
}

// Implementation of PartialEq<&str> for Column
impl<'a> PartialEq<&str> for Column<'a> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Column::Text(text) => text == other,
            Column::SubQuery(sub_query, _) => sub_query.to_sql() == *other,
        }
    }
}
