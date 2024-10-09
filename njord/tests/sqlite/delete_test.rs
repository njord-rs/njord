use super::User;
use njord::column::Column;
use njord::condition::{Condition, Value};
use njord::sqlite;
use njord::sqlite::select::SelectQueryBuilder;
use std::collections::HashMap;
use std::path::Path;

#[test]
fn delete() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let condition = Condition::Eq(
        "address".to_string(),
        Value::Literal("Some Random Address 1".to_string()),
    );

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(ref c) => {
            let result = sqlite::delete(c)
                .from(User::default())
                .where_clause(condition)
                .order_by(order)
                .limit(20)
                .offset(0)
                .build();
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

#[test]
fn delete_with_subquery() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(ref c) => {
            let sub_query =
                SelectQueryBuilder::new(c, vec![Column::<User>::Text("username".to_string())])
                    .where_clause(Condition::Eq(
                        "id".to_string(),
                        Value::Literal(1.to_string()),
                    ))
                    .limit(1);

            let condition =
                Condition::Eq("address".to_string(), Value::Subquery(Box::new(sub_query)));

            let result = sqlite::delete(c)
                .from(User::default())
                .where_clause(condition)
                .build();
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}
