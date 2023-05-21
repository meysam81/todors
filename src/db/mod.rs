use crate::errors;

#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "sqlite")]
pub use sqlite::*;

impl std::convert::From<sqlx::migrate::MigrateError> for errors::TodoErrors {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        errors::TodoErrors::DatabaseError(err.into())
    }
}

pub async fn connect(conn_str: &str, max_conn: Option<u32>) -> Result<Pool, errors::TodoErrors> {
    let pool = init_pool(conn_str, max_conn).await?;
    apply_migrations(&pool).await?;
    Ok(pool)
}
