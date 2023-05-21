use crate::errors;

#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "sqlite")]
pub use sqlite::*;

pub async fn connect(conn_str: &str, max_conn: Option<u32>) -> Result<Pool, errors::TodoErrors> {
    let pool = init_pool(conn_str, max_conn).await?;
    apply_migrations(&pool).await?;
    Ok(pool)
}
