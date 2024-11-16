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

use oracle::{Connection, Error};

pub mod delete;
pub mod error;
pub mod insert;
pub mod select;
pub mod update;
mod util;

pub use delete::delete;
pub use error::OracleError;
pub use insert::insert;
pub use select::select;
pub use update::update;

/// Open a database connection.
///
/// This function opens a connection to a Oracle database located at the specified path.
///
/// # Arguments
///
/// * `username` - A reference to the username for the Oracle database.
/// * `password` - A reference to the password for the Oracle database.
/// * `connect_string` - A reference to the connect string for the Oracle database.
///
/// # Returns
///
/// Returns a `Result` containing a `PooledConn` if the operation was successful, or an `Error` if an error occurred.
pub fn open(username: &str, password: &str, connect_string: &str) -> Result<Connection, Error> {
    let conn = Connection::connect(username, password, connect_string);

    match conn {
        Ok(conn) => {
            println!("Successfully connected to Oracle database");

            return Ok(conn);
        }
        Err(err) => {
            eprintln!("Error: {}", err);

            return Err(err);
        }
    }
}
