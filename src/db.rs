use crate::consts;

pub use sqlx::sqlite::SqlitePool as Pool;
use sqlx::sqlite::SqlitePoolOptions;
pub use sqlx::sqlite::SqliteQueryResult as QueryResult;
pub use sqlx::{query, query_as, Error, FromRow, Row};

pub async fn connect(conn_str: &str, max_conn: Option<u32>) -> Result<Pool, sqlx::Error> {
    let max_conn = max_conn.unwrap_or(consts::DEFAULT_DB_CONNECTION_POOL_SIZE);

    let conn = match conn_str {
        ":memory:" => {
            SqlitePoolOptions::new()
                .max_connections(max_conn)
                .connect(conn_str)
                .await?
        }
        conn if conn.starts_with("sqlite://") => {
            let conn = conn.strip_prefix("sqlite://").unwrap();
            let conn_path = std::path::Path::new(conn);
            if !conn_path.exists() {
                let parent_dir = conn_path.parent().unwrap();
                std::fs::create_dir_all(parent_dir).unwrap();
                std::fs::File::create(conn).unwrap();
            }

            SqlitePoolOptions::new()
                .max_connections(max_conn)
                .connect(conn_str)
                .await?
        }
        _ => unimplemented!("This connection string is not supported: `{}`", conn_str),
    };

    sqlx::migrate!("./migrations").run(&conn).await?;

    Ok(conn)
}
