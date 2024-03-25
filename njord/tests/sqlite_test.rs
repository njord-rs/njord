// integrations tests for sqlite

use std::path::Path;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use njord::condition::Condition;
use njord::sqlite::{self};
use njord::table::Table;
use njord_derive::Table;

mod common;

#[derive(Table)]
#[table_name = "users"]
pub struct User {
    id: usize,
    username: String,
    email: String,
    address: String,
}

// #[derive(Table)]
// #[table_name = "categories"]
// pub struct Category {
//     id: usize,
//     name: String,
// }
//
// #[derive(Table)]
// #[table_name = "products"]
// pub struct Product {
//     id: usize,
//     name: String,
//     description: String,
//     price: f64,
//     stock_quantity: usize,
//     category: Category,     // one-to-one relationship
//     discount: Option<f64>,
// }
//
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

    println!("{:?}", db_path);

    let result = sqlite::open(db_path);
    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let db_relative_path = "./db/insert_row.db";
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
            let result = sqlite::insert(c, &table_row);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to drop table: {:?}", e);
        }
    }
}

#[test]
fn select() {
    let db_relative_path = "./db/select.db";
    let db_path = Path::new(&db_relative_path);
    let conn = sqlite::open(db_path);

    let columns = vec!["username".to_string(), "address".to_string()];
    let condition = Condition::Eq("username".to_string(), "mjovanc".to_string());

    // TODO: fix the issue with sqlite::select()
    // match conn {
    //     Ok(c) => {
    //         let result = sqlite::select(c, columns)
    //             .from(&User::default())
    //             .where_clause(condition)
    //             .build::<User>();
    //
    //         match result {
    //             Ok(r) => {
    //                 println!("\nSELECT ROWS: ");
    //                 // print_rows(&result);
    //                 assert_eq!(r.len(), 2);
    //             },
    //             Err(e) => panic!("Failed to SELECT: {:?}", e),
    //         };
    //     }
    //     Err(e) => panic!("Failed to select: {:?}", e),
    // };
}

// #[test]
// fn select_distinct() {
//     let db_name = "select_distinct.db";
//     let _ = common::drop_db_sqlite(db_name);
//     let conn = common::open_db_sqlite(db_name).unwrap();
//     let init_tables_result = common::initialize_tables_sqlite(db_name);
//     common::insert_rows_sqlite_distinct(db_name).expect("Failed to insert rows to sqlite.");

//     let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
//     let condition = Condition::Eq(
//         "description".to_string(),
//         "Some description for Table A".to_string(),
//     );

//     match init_tables_result {
//         Ok(_) => {
//             let result = sqlite::select(conn, columns)
//                 .from(&TableA::default())
//                 .where_clause(condition)
//                 .distinct()
//                 .build::<TableA>();

//             match result {
//                 Ok(result) => {
//                     println!("\nSELECT DISTINCT ROWS: ");
//                     // print_rows(&result);
//                     assert_eq!(result.len(), 1);
//                 },
//                 Err(error) => panic!("Failed to SELECT: {:?}", error),
//             };
//         }
//         Err(error) => panic!("Failed to select: {:?}", error),
//     };
// }

// #[test]
// fn select_group_by() {
//     let db_name = "select_group_by.db";
//     let _ = common::drop_db_sqlite(db_name);
//     let conn = common::open_db_sqlite(db_name).unwrap();
//     let init_tables_result = common::initialize_tables_sqlite(db_name);
//     common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

//     let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
//     let condition = Condition::Eq(
//         "description".to_string(),
//         "Some description for Table A".to_string(),
//     );
//     let group_by = vec![
//         "description".to_string(),
//         "amount".to_string(),
//     ];

//     match init_tables_result {
//         Ok(_) => {
//             let result = sqlite::select(conn, columns)
//                 .from(&TableA::default())
//                 .where_clause(condition)
//                 .group_by(group_by)
//                 .build::<TableA>();

//             match result {
//                 Ok(result) => {
//                     println!("\nSELECT GROUP BY ROWS: ");
//                     // print_rows(&result);
//                     assert_eq!(result.len(), 2);
//                 },
//                 Err(error) => panic!("Failed to SELECT: {:?}", error),
//             };
//         }
//         Err(error) => panic!("Failed to select: {:?}", error),
//     };
// }

// #[test]
// fn select_order_by() {
//     let db_name = "select_order_by.db";
//     let _ = common::drop_db_sqlite(db_name);
//     let conn = common::open_db_sqlite(db_name).unwrap();
//     let init_tables_result = common::initialize_tables_sqlite(db_name);
//     common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

//     let columns = vec!["title".to_string(), "description".to_string(), "amount".to_string()];
//     let condition = Condition::Eq(
//         "description".to_string(),
//         "Some description for Table A".to_string(),
//     );
//     let group_by = vec![
//         "description".to_string(),
//         "amount".to_string(),
//     ];
//     let mut order_by = HashMap::new();
//     order_by.insert(vec!["amount".to_string()], "DESC".to_string());
//     order_by.insert(vec!["description".to_string()], "ASC".to_string());

//     match init_tables_result {
//         Ok(_) => {
//             let result = sqlite::select(conn, columns)
//                 .from(&TableA::default())
//                 .where_clause(condition)
//                 .order_by(order_by)
//                 .group_by(group_by)
//                 .build::<TableA>();

//             match result {
//                 Ok(result) => {
//                     println!("\nSELECT ORDER BY ROWS: ");
//                     // print_rows(&result);
//                     assert_eq!(result.len(), 2);
//                 },
//                 Err(error) => panic!("Failed to SELECT: {:?}", error),
//             };
//         }
//         Err(error) => panic!("Failed to select: {:?}", error),
//     };
// }

// #[test]
// fn select_limit_offset() {
//     let db_name = "select_limit_offset.db";
//     // let _ = common::drop_db_sqlite(db_name);
//     let conn = common::open_db_sqlite(db_name).unwrap();
//     // let init_tables_result = common::initialize_tables_sqlite(db_name);
//     // common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

//     let columns = vec![
//         "title".to_string(),
//         "description".to_string(),
//         "amount".to_string(),
//     ];
//     let condition = Condition::Eq(
//         "description".to_string(),
//         "Some description for Table A".to_string(),
//     );
//     let group_by = vec!["description".to_string(), "amount".to_string()];
//     let mut order_by = HashMap::new();
//     order_by.insert(vec!["amount".to_string()], "ASC".to_string());
//     order_by.insert(vec!["description".to_string()], "DESC".to_string());

//     match init_tables_result {
//         Ok(_) => {
//             //TODO we should probably get back a vector of the table that was used so we can more
//             // easily pass around that struct in the code
//             let result = sqlite::select(conn, columns)
//                 .from(&TableA::default())
//                 .where_clause(condition)
//                 .order_by(order_by)
//                 .group_by(group_by)
//                 .limit(1)
//                 .offset(1)
//                 .build::<TableA>();

//             match result {
//                 Ok(result) => {
//                     println!("\nSELECT LIMIT & OFFSET ROWS: ");
//                     // print_rows(&result);
//                     assert_eq!(result.len(), 1);
//                 }
//                 Err(error) => panic!("Failed to SELECT: {:?}", error),
//             };
//         }
//         Err(error) => panic!("Failed to select: {:?}", error),
//     };
// }

// #[test]
// fn select_having() {
//     let db_name = "select_having.db";
//     // let _ = common::drop_db_sqlite(db_name);
//     let conn = common::open_db_sqlite(db_name).unwrap();
//     // let init_tables_result = common::initialize_tables_sqlite(db_name);
//     // common::insert_rows_sqlite(db_name).expect("Failed to insert rows to sqlite.");

//     let columns = vec![
//         "title".to_string(),
//         "description".to_string(),
//         "amount".to_string(),
//     ];
//     let where_condition = Condition::Eq(
//         "description".to_string(),
//         "Some description for Table A".to_string(),
//     );
//     let group_by = vec!["description".to_string(), "amount".to_string()];
//     let mut order_by = HashMap::new();
//     order_by.insert(vec!["amount".to_string()], "ASC".to_string());
//     order_by.insert(vec!["description".to_string()], "DESC".to_string());
//     let having_condition = Condition::Gt("amount".to_string(), "10".to_string());

//     let result = sqlite::select(conn, columns)
//         .from(TableA::default())
//         .where_clause(where_condition)
//         .order_by(order_by)
//         .group_by(group_by)
//         .having(having_condition)
//         .build();

//     match result {
//         Ok(result) => {
//             println!("\nSELECT HAVING: ");
//             // print_rows(&result);
//             assert_eq!(result.len(), 1);
//         }
//         Err(error) => panic!("Failed to SELECT: {:?}", error),
//     };
// }
