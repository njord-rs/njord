//! BSD 3-Clause License
//!
//! Copyright (c) 2024,
//!     Marcus Cvjeticanin
//!     Chase Willden
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use mysql::{prelude::Queryable, Error, Pool, PooledConn};

pub mod delete;
pub mod error;
pub mod insert;
pub mod select;
pub mod update;
mod util;

pub use delete::delete;
pub use error::MySqlError;
pub use insert::insert;
pub use select::select;
pub use update::update;

/// Open a database connection.
///
/// This function opens a connection to a MySql database located at the specified path.
///
/// # Arguments
///
/// * `db_path` - A reference to the path where the MySql database is located.
///
/// # Returns
///
/// Returns a `Result` containing a `PooledConn` if the operation was successful, or an `Error` if an error occurred.
pub fn open(url: &str) -> Result<PooledConn, Error> {
    let pool = Pool::new(url)?;

    let conn = pool.get_conn()?;

    Ok(conn)
}

/// Executes a raw SQL query and returns a vector of table rows.
///
/// # Arguments
///
/// * `sql` - The SQL query to execute.
/// * `conn` - A reference to the database connection.
///
/// # Returns
///
/// A `Result` containing a vector of table rows if successful,
/// or a `rusqlite::Error` if an error occurs during the execution.
pub fn raw_execute(conn: &mut PooledConn, sql: &str) -> Result<(), MySqlError> {
    match conn.query_drop(sql) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
