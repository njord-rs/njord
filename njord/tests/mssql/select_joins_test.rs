use njord::condition::Condition;
use njord::keys::AutoIncrementPrimaryKey;
use njord::mssql;
use njord::table::Table;
use njord::util::JoinType;
use njord::{column::Column, condition::Value};
use std::sync::Arc;

use crate::{Category, CategoryWithJoin, Product};

async fn insert_mock_data<T: Table + Clone + Default>(table_rows: Vec<T>) {
    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    match conn {
        Ok(ref mut c) => {
            let result = mssql::insert(c, table_rows).await;
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

async fn delete_mock_data<T: Table + Clone + Default>(names: Vec<String>, column: String) {
    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    match conn {
        Ok(ref mut c) => {
            // Transform Vec<String> into Vec<Value>
            let value_list: Vec<Value> = names
                .into_iter()
                .map(Value::Literal) // Wrap each username as a Value::Literal
                .collect();

            let result = mssql::delete(c)
                .from(T::default())
                .where_clause(Condition::In(column, value_list))
                .build()
                .await;
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

#[tokio::test]
async fn select_inner_join() {
    insert_mock_data(vec![Category {
        id: AutoIncrementPrimaryKey::new(Some(2)),
        name: "select_inner_join_test".to_string(),
    }])
    .await;

    insert_mock_data(vec![Product {
        id: AutoIncrementPrimaryKey::new(Some(2)),
        name: "select_inner_join_test".to_string(),
        description: "select_inner_join_test".to_string(),
        price: 10.0,
        stock_quantity: 10,
        discount: 0.0,
        category_id: 2,
    }])
    .await;

    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    // Assume we have pre-inserted some data into the users and products tables
    let columns = vec![
        Column::Text("categories.name".to_string()),
        Column::Text("products.name".to_string()),
        Column::Text("products.price".to_string()),
    ];

    // Assuming a hypothetical join condition: users.id = products.user_id
    let join_condition = Condition::Eq(
        "categories.id".to_string(),
        Value::Literal("products.category_id".to_string()),
    );
    match conn {
        Ok(ref mut c) => {
            let result = mssql::select(columns)
                .from(CategoryWithJoin::default())
                .join(
                    JoinType::Inner,
                    Arc::new(Product::default()),
                    join_condition,
                )
                .build(c)
                .await;
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

    delete_mock_data::<Category>(
        vec!["select_inner_join_test".to_string()],
        "name".to_string(),
    )
    .await;

    delete_mock_data::<Product>(
        vec!["select_inner_join_test".to_string()],
        "name".to_string(),
    )
    .await;
}

#[tokio::test]
async fn select_left_join() {
    insert_mock_data(vec![Category {
        id: AutoIncrementPrimaryKey::new(Some(1)),
        name: "select_inner_join_test".to_string(),
    }])
    .await;

    insert_mock_data(vec![Product {
        id: AutoIncrementPrimaryKey::new(Some(1)),
        name: "select_inner_join_test".to_string(),
        description: "select_inner_join_test".to_string(),
        price: 10.0,
        stock_quantity: 10,
        discount: 0.0,
        category_id: 1,
    }])
    .await;

    let connection_string =
        "jdbc:sqlserver://localhost;encrypt=true;username=sa;password=Njord_passw0rd;databaseName=NjordDatabase;";
    let mut conn = mssql::open(connection_string).await;

    // Assume we have pre-inserted some data into the users and products tables
    let columns = vec![
        Column::Text("categories.name".to_string()),
        Column::Text("products.name".to_string()),
        Column::Text("products.price".to_string()),
    ];

    // Assuming a hypothetical join condition: users.id = products.user_id
    let join_condition = Condition::Eq(
        "categories.id".to_string(),
        Value::Literal("products.category_id".to_string()),
    );
    match conn {
        Ok(ref mut c) => {
            let result = mssql::select(columns)
                .from(CategoryWithJoin::default())
                .join(JoinType::Left, Arc::new(Product::default()), join_condition)
                .build(c)
                .await;
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

    delete_mock_data::<Category>(
        vec!["select_inner_join_test".to_string()],
        "name".to_string(),
    )
    .await;

    delete_mock_data::<Product>(
        vec!["select_inner_join_test".to_string()],
        "name".to_string(),
    )
    .await;
}
