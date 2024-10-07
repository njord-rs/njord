use njord::condition::Condition;
use njord::sqlite;
use std::collections::HashMap;
use std::path::Path;
use super::User;

#[test]
fn delete() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let condition = Condition::Eq("address".to_string(), "Some Random Address 1".to_string());

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
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