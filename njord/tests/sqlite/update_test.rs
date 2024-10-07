use njord::column::Column;
use njord::condition::Condition;
use njord::keys::AutoIncrementPrimaryKey;
use njord::sqlite::select::SelectQueryBuilder;
use njord::sqlite::{self};
use std::collections::HashMap;
use std::path::Path;
use super::{User};

#[test]
fn update() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec!["username".to_string()];

    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::<usize>::new(Some(0)),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::update(&c, table_row)
                .set(columns)
                .where_clause(condition)
                .order_by(order)
                .limit(4)
                .offset(0)
                .build();
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to UPDATE: {:?}", e);
        }
    }
}

#[test]
fn update_with_sub_queries() {
    let db_relative_path = "./db/update.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::<usize>::new(Some(0)),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    let columns = vec!["username".to_string()];

    match conn {
        Ok(c) => {
            let sub_query = SelectQueryBuilder::new(&c, vec![Column::Text("email".to_string())])
                .from(User::default())
                .where_clause(Condition::Eq(
                    "email".to_string(),
                    "mjovanc@icloud.com".to_string(),
                ))
                .limit(1);

            let set_subqueries = HashMap::from([("email".to_string(), sub_query)]);

            let result = sqlite::update(&c, table_row)
                .set(columns)
                .set_subqueries(set_subqueries)
                .where_clause(Condition::Eq("username".to_owned(), "otheruser".to_owned()))
                .build();

            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => panic!("Failed to UPDATE: {:?}", e),
    };
}