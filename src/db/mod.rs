#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "sqlite")]
mod _sqlite {
    use super::sqlite;

    pub use self::sqlite::{connect, query, query_as, Error, FromRow, Pool, QueryResult, Row};
}

#[cfg(feature = "sqlite")]
pub use self::_sqlite::*;
