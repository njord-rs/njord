use std::collections::HashMap;

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
    fn get_columns(&self) -> &HashMap<String, String>;

    /// Get the names of the columns.
    ///
    /// Returns a `Vec<String>` containing the names of the columns in the same order
    /// as they appear in the table.
    fn get_column_fields(&self) -> Vec<String>;
}

#[test]
fn create_tables() {
    // create table 1
    #[derive(Table, Debug)]
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

    // create table 2
    #[derive(Table, Debug)]
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

    // create table 3
    #[derive(Table, Debug)]
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
        ("title".to_string(), "String".to_string()),
        ("desc".to_string(), "String".to_string()),
        ("amount".to_string(), "u32".to_string()),
    ]
    .into_iter()
    .collect();

    let expected_columns_b: HashMap<String, String> = vec![
        ("name".to_string(), "String".to_string()),
        ("age".to_string(), "u32".to_string()),
        ("email".to_string(), "String".to_string()),
    ]
    .into_iter()
    .collect();

    let expected_columns_c: HashMap<String, String> = vec![
        ("product_id".to_string(), "i64".to_string()),
        ("product_name".to_string(), "String".to_string()),
        ("price".to_string(), "f64".to_string()),
        ("in_stock".to_string(), "bool".to_string()),
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
    //TODO need to fix these later
    // assert_eq!(table_a.get_columns(), expected_columns_a);
    // assert_eq!(table_b.get_columns(), expected_columns_b);
    // assert_eq!(table_c.get_columns(), expected_columns_c);

    // assert that we get the correct table fields
    assert_eq!(table_a.get_column_fields(), expected_fields_a);
    assert_eq!(table_b.get_column_fields(), expected_fields_b);
    assert_eq!(table_c.get_column_fields(), expected_fields_c);

    // assert that we get the correct table name
    assert_eq!(table_a.get_name(), "TableA");
    assert_eq!(table_b.get_name(), "TableB");
    assert_eq!(table_c.get_name(), "TableC");
}
