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

    let table_a = Box::new(TableA::default());
    let table_b = Box::new(TableB::default());
    let table_c = Box::new(TableC::default());

    let mut tables: Vec<Box<dyn Table>> = Vec::new();
    tables.push(table_a);
    tables.push(table_b);
    tables.push(table_c);

    tables
}
