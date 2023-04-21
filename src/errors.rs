use crate::db::Error as DbError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TodoErrors {
    #[error("database error")]
    DatabaseError(#[from] DbError),

    #[error("no update")]
    NoUpdate,
}
