// use std::{env, fs, vec};

// use njord::{sqlite, table::Table};

// // #[cfg(feature = "derive")]
// use njord_derive::Table;
// use rusqlite::types::Value;
// use rusqlite::{Connection, Error};

// #[derive(Table, Debug, Default)]
// struct TableA {
//     title: String,
//     description: String,
//     amount: u32,
// }

// #[derive(Table, Debug, Default)]
// struct TableB {
//     name: String,
//     age: u32,
//     email: String,
// }

// #[derive(Table, Debug, Default)]
// struct TableC {
//     product_id: i64,
//     product_name: String,
//     price: f64,
//     in_stock: bool,
// }

pub fn open_db_sqlite(db_name: &str) -> Result<Connection, Error> {
    let conn = sqlite::open(db_name).unwrap();
    Ok(conn)
}

// pub fn drop_db_sqlite(db_name: &str) -> Result<(), std::io::Error> {
//     let target_dir = env::var("OUT_DIR").unwrap_or_else(|_| "../target".to_string());
//     let db_file_path = format!("{}/{}", target_dir, db_name);
//     fs::remove_file(db_file_path)
// }

// // pub fn initialize_tables_sqlite(db_name: &str) -> Result<(), Error> {
// //     let conn = open_db_sqlite(db_name).unwrap();
// //     let tables = generate_tables_sqlite();
// //     let result = sqlite::init(conn, tables);

// //     result
// // }

// pub fn insert_rows_sqlite(db_name: &str) -> Result<(), Error> {
//     let conn1 = open_db_sqlite(db_name)?;
//     let conn2 = open_db_sqlite(db_name)?;

//     let table_row_1: TableA = TableA {
//         title: "Item 1".to_string(),
//         description: "Some description for Table A".to_string(),
//         amount: 10,
//     };

//     let table_row_2: TableA = TableA {
//         title: "Item 2".to_string(),
//         description: "Some description for Table A".to_string(),
//         amount: 20,
//     };

//     sqlite::insert(conn1, &table_row_1).expect("Failed to insert table_row_1");
//     let result = sqlite::insert(conn2, &table_row_2);

//     assert!(result.is_ok());

//     result
// }

// pub fn insert_rows_sqlite_distinct(db_name: &str) -> Result<(), Error> {
//     let conn1 = open_db_sqlite(db_name)?;
//     let conn2 = open_db_sqlite(db_name)?;

//     let table_row_1: TableA = TableA {
//         title: "Item".to_string(),
//         description: "Some description for Table A".to_string(),
//         amount: 10,
//     };

//     let table_row_2: TableA = TableA {
//         title: "Item".to_string(),
//         description: "Some description for Table A".to_string(),
//         amount: 10,
//     };

//     sqlite::insert(conn1, &table_row_1).expect("Failed to insert table_row_1");
//     let result = sqlite::insert(conn2, &table_row_2);

//     assert!(result.is_ok());

//     result
// }

// pub fn generate_tables_sqlite() -> Vec<Box<dyn Table>> {
//     let table_a = Box::<TableA>::default();
//     let table_b = Box::<TableB>::default();
//     let table_c = Box::<TableC>::default();

//     let tables: Vec<Box<dyn Table>> = vec![table_a, table_b, table_c];

//     tables
// }

// // pub fn print_rows(result: &Vec<Row>) {
// //     for row in result {
// //         for (column, value) in row {
// //             match value {
// //                 Value::Null => println!("\t{}: NULL", column),
// //                 Value::Integer(i) => println!("\t{}: {}", column, i),
// //                 Value::Real(f) => println!("\t{}: {}", column, f),
// //                 Value::Text(s) => println!("\t{}: {}", column, s),
// //                 Value::Blob(b) => println!("\t{}: <blob of length {}>", column, b.len()),
// //             }
// //         }
// //         println!("\t---");
// //     }
// // }
