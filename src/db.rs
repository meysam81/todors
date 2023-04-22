use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
pub use sqlx::sqlite::{SqlitePool as Pool, SqliteQueryResult as QueryResult};
pub use sqlx::{query, query_as, Error, FromRow};

pub async fn connect(conn: &str, max_conn: Option<u32>) -> Result<SqlitePool, sqlx::Error> {
    let max_conn = max_conn.unwrap_or(5);

    if !std::path::Path::new(conn).exists() {
        std::fs::File::create(conn)?;
    }

    let c = SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(conn)
        .await?;

    migrate(&c).await?;

    Ok(c)
}

async fn migrate(conn: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(conn).await?;
    Ok(())
}
