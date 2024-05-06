// integrations tests for sqlite

use njord::condition::Condition;
use njord::sqlite::{self};
use njord::table::Table;
use njord_derive::Table;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::path::Path;

#[derive(Table)]
#[table_name = "users"]
pub struct User {
    id: usize, // TODO: should we create some sort of PrimaryKey<usize> ?
    username: String,
    email: String,
    address: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct Category {
    id: usize,
    name: String,
}

#[derive(Table)]
#[table_name = "products"]
pub struct Product {
    id: usize,
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    category: Category, // one-to-one relationship
    discount: f64,
}

// #[derive(Table)]
// #[table_name = "orders"]
// pub struct Order {
//     id: usize,
//     user: User,             // one-to-one relationship
//     products: Vec<Product>, // one-to-many relationship - populates from based on junction table (gets from macro attribute "table_name" and combines them for example, orders_products)
//     total_cost: f64,
// }

#[test]
fn open_db() {
    let db_relative_path = "./db/open.db";
    let db_path = Path::new(&db_relative_path);

    let result = sqlite::open(db_path);
    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    // generate random number
    let mut rng = StdRng::from_entropy();
    let max_usize = usize::MAX;
    let random_number: usize = rng.gen_range(0..max_usize / 2);

    let table_row: User = User {
        id: random_number,
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(c) => {
            let result = sqlite::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn update() {
    let db_relative_path = "./db/insert.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns: Vec<String> = vec!["address".to_string()];

    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    let table_row: User = User {
        id: 0,
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::update(c, table_row)
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

#[test]
fn select() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        "id".to_string(),
        "username".to_string(),
        "email".to_string(),
        "address".to_string(),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(c, columns)
                .from(User::default())
                .where_clause(condition)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 2),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_distinct() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        "id".to_string(),
        "username".to_string(),
        "email".to_string(),
        "address".to_string(),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(c, columns)
                .from(User::default())
                .where_clause(condition)
                .distinct()
                .build();

            match result {
                Ok(r) => {
                    // TODO: this test does not work properly since it should return 1 but it seems
                    // like it returns all rows because id is different. Need to check up on that.
                    assert_eq!(r.len(), 2);
                }
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_group_by() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        "id".to_string(),
        "username".to_string(),
        "email".to_string(),
        "address".to_string(),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    match conn {
        Ok(c) => {
            let result = sqlite::select(c, columns)
                .from(User::default())
                .where_clause(condition)
                .group_by(group_by)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_order_by() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        "id".to_string(),
        "username".to_string(),
        "email".to_string(),
        "address".to_string(),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["email".to_string()], "ASC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(c, columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_limit_offset() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        "id".to_string(),
        "username".to_string(),
        "email".to_string(),
        "address".to_string(),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(c, columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .limit(1)
                .offset(0)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(error) => panic!("Failed to SELECT: {:?}", error),
    };
}

#[test]
fn select_having() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec![
        "id".to_string(),
        "username".to_string(),
        "email".to_string(),
        "address".to_string(),
    ];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["email".to_string()], "DESC".to_string());

    let having_condition = Condition::Gt("id".to_string(), "1".to_string());

    match conn {
        Ok(c) => {
            let result = sqlite::select(c, columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .having(having_condition)
                .build();

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}
