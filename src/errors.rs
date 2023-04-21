use crate::db::Error as DbError;
use crate::serializers::Error as SerializerError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TodoErrors {
    #[error("database error")]
    DatabaseError(#[from] DbError),

    #[error("no update")]
    NoUpdate,

    #[error("serializer error")]
    SerializerError(#[from] SerializerError),
}
