use std::vec;

use njord::table::Table;
use njord_derive::Table;

pub fn initialized_tables_sqlite() -> Vec<Box<dyn Table>> {
    #[derive(Table, Debug, Default)]
    struct TableA {
        title: String,
        description: String,
        amount: u32,
    }

    #[derive(Table, Debug, Default)]
    struct TableB {
        name: String,
        age: u32,
        email: String,
    }

    #[derive(Table, Debug, Default)]
    struct TableC {
        product_id: i64,
        product_name: String,
        price: f64,
        in_stock: bool,
    }

    let table_a = Box::<TableA>::default();
    let table_b = Box::<TableB>::default();
    let table_c = Box::<TableC>::default();

    let tables: Vec<Box<dyn Table>> = vec![table_a, table_b, table_c];

    tables
}
