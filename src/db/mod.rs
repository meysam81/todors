mod sqlite;

#[cfg(feature = "sqlite")]
mod _sqlite {
    use super::*;

    pub use self::sqlite::connect;
    pub use sqlx::sqlite::SqlitePool as Pool;
    pub use sqlx::sqlite::SqliteQueryResult as QueryResult;
    pub use sqlx::{query, query_as, Error, FromRow, Row};
}

#[cfg(feature = "sqlite")]
pub use self::_sqlite::*;
