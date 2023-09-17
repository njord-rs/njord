// re-export structs and traits
pub use self::table::Table;
pub use self::table::TableStruct;

// re-export functions
pub use self::init::init;

// submodules
pub mod init;
pub mod table;

// select
// Vector of strings of fields and struct name
// fn select() -> Result<()> {}

// update
// initilized struct (table) and condition as string as argument

// delete
// table struct and condition (string) as argument

// insert into
// insert new record/row into table
// "initilized" struct (table) as argument that contains column names and values

// create table
// should take a struct as argument so we know the column names and value types
