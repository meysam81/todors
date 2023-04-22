use crate::db::Error as DbError;
use crate::serializers::Error as SerializerError;
use std::io::Error as IoError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TodoErrors {
    #[error("database error")]
    DatabaseError(#[from] DbError),

    #[error("no update")]
    NoUpdate,

    #[error("serializer error")]
    SerializerError(#[from] SerializerError),

    #[error("io error")]
    IoError(#[from] IoError),
}
