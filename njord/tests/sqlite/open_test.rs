use njord::keys::{AutoIncrementPrimaryKey, PrimaryKey};
use njord::sqlite::{self};
use njord::table::Table;
use njord_derive::Table;
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
fn open_db() {
    let db_relative_path = "./db/open.db";
    let db_path = Path::new(&db_relative_path);

    let result = sqlite::open(db_path);
    assert!(result.is_ok());
}