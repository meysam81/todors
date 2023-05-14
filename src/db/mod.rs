#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "sqlite")]
pub use sqlite::*;

pub async fn connect(conn_str: &str, max_conn: Option<u32>) -> Result<Pool, sqlx::Error> {
    let pool = init_pool(conn_str, max_conn).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
