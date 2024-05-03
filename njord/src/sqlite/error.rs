use rusqlite::Error as RusqliteError;

#[derive(Debug)]
pub enum SqliteError {
    SelectError(RusqliteError),
    InsertError(RusqliteError),
    UpdateError(RusqliteError),
    DeleteError(RusqliteError),
}

impl From<RusqliteError> for SqliteError {
    fn from(error: RusqliteError) -> Self {
        SqliteError::InsertError(error)
    }
}
