#[derive(Table, Default)]
pub struct User {
    user_id: usize,
    username: String,
    email: String,
    address: String,
}

#[derive(Table, Default)]
pub struct Product {
    product_id: usize,
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    category: String,
}

#[derive(Table, Default)]
pub struct Order {
    order_id: usize,
    user_id: usize,
    products: Vec<Product>,
    total_cost: f64,
    order_date: NaiveDateTime,
}
