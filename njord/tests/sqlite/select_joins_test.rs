use njord::condition::Condition;
use njord::sqlite;
use njord::util::JoinType;
use njord::{column::Column, condition::Value};
use std::path::Path;
use std::sync::Arc;

use crate::{Product, UsersWithJoin};

#[test]
fn select_inner_join() {
    let db_relative_path = "./db/select_join.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    // Assume we have pre-inserted some data into the users and products tables
    let columns = vec![
        Column::Text("users.username".to_string()),
        Column::Text("products.name".to_string()),
        Column::Text("products.price".to_string()),
    ];

    // Assuming a hypothetical join condition: users.id = products.user_id
    let join_condition = Condition::Eq(
        "users.id".to_string(),
        Value::Literal("products.user_id".to_string()),
    );
    match conn {
        Ok(ref c) => {
            let result = sqlite::select(columns)
                .from(UsersWithJoin::default())
                .join(
                    JoinType::Inner,
                    Arc::new(Product::default()),
                    join_condition,
                )
                .build(c);
            match result {
                Ok(r) => {
                    // Check the number of results and assert against expected values
                    assert!(!r.is_empty(), "Expected results, but got none.");
                    // Further assertions on expected data can be made here based on inserted data
                }
                Err(e) => panic!("Failed to SELECT with JOIN: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}

#[test]
fn select_left_join() {
    let db_relative_path = "./db/select_join.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    // Assume we have pre-inserted some data into the users and products tables
    let columns = vec![
        Column::Text("users.username".to_string()),
        Column::Text("products.name".to_string()),
        Column::Text("products.price".to_string()),
    ];

    // Assuming a hypothetical join condition: users.id = products.user_id
    let join_condition = Condition::Eq(
        "users.id".to_string(),
        Value::Literal("products.user_id".to_string()),
    );
    match conn {
        Ok(ref c) => {
            let result = sqlite::select(columns)
                .from(UsersWithJoin::default())
                .join(JoinType::Left, Arc::new(Product::default()), join_condition)
                .build(c);
            match result {
                Ok(r) => {
                    // Check the number of results and assert against expected values
                    assert!(!r.is_empty(), "Expected results, but got none.");
                    assert_eq!(r.len(), 2, "Expected 2 results from the LEFT JOIN query.");
                    // Further assertions on expected data can be made here based on inserted data
                }
                Err(e) => panic!("Failed to SELECT with JOIN: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}
