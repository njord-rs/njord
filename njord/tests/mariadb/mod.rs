mod delete_test;
mod insert_test;
mod open_test;
mod select_joins_test;
mod select_test;
mod update_test;

use njord::keys::AutoIncrementPrimaryKey;
use njord::table::Table;
use njord_derive::Table;

#[derive(Table, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub username: String,
    pub email: String,
    pub address: String,
}

#[derive(Table, Clone)]
#[table_name = "users"]
pub struct UserWithSubQuery {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub username: String,
    pub email: String,
    pub address: String,
    pub additional_address: String,
}

#[derive(Table, Clone)]
#[table_name = "categories"]
pub struct Category {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub name: String,
}

#[derive(Table, Clone)]
#[table_name = "products"]
pub struct Product {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock_quantity: usize,
    // pub category: Category, // one-to-one relationship
    pub category_id: usize,
    pub discount: f64,
}

#[derive(Table)]
#[table_name = "users"]
pub struct UsersWithJoin {
    username: String,
    price: f64,
    name: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct CategoryWithJoin {
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    discount: f64,
    category_name: String,
}
