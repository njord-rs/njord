use njord::condition::Condition;
use njord::keys::{AutoIncrementPrimaryKey, PrimaryKey};
use njord::sqlite::{self};
use njord::table::Table;
use njord_derive::Table;
use std::collections::HashMap;
use std::path::Path;

#[derive(Table, Clone)]
#[table_name = "users"]
pub struct User {
    id: AutoIncrementPrimaryKey<usize>,
    username: String,
    email: String,
    address: String,
}

#[derive(Table)]
#[table_name = "users"]
pub struct UserWithSubQuery {
    id: AutoIncrementPrimaryKey<usize>,
    username: String,
    email: String,
    address: String,
    additional_address: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct Category {
    id: PrimaryKey<usize>,
    name: String,
}

#[derive(Table)]
#[table_name = "products"]
pub struct Product {
    id: PrimaryKey<usize>,
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    category: Category, // one-to-one relationship
    discount: f64,
}

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