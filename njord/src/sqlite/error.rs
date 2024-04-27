use rusqlite::Error as RusqliteError;

#[derive(Debug)]
pub enum SqliteError {
    InsertError(RusqliteError),
}

impl From<RusqliteError> for SqliteError {
    fn from(error: RusqliteError) -> Self {
        SqliteError::InsertError(error)
    }
}
