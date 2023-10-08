use crate::table::Table;
use crate::util::convert_insert_values;

use log::info;
use rusqlite::{Connection, Result};
use std::fmt::Error;

pub fn select(mut conn: Connection, table_row: &dyn Table) -> Result<()> {}
