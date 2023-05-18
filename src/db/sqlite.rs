use crate::consts;

pub use sqlx::sqlite::SqliteError as DriverError;
pub use sqlx::sqlite::SqlitePool as Pool;
use sqlx::sqlite::SqlitePoolOptions;
pub use sqlx::sqlite::SqliteQueryResult as QueryResult;
pub use sqlx::{query, query_as, Error, FromRow, Row};

pub async fn init_pool(conn_str: &str, max_conn: Option<u32>) -> Result<Pool, sqlx::Error> {
    let max_conn = max_conn.unwrap_or(consts::DEFAULT_DB_CONNECTION_POOL_SIZE);

    let conn = match conn_str {
        ":memory:" => get_sqlite_conn(conn_str, max_conn).await?,
        conn if conn.starts_with("sqlite://") => {
            let conn = conn.strip_prefix("sqlite://").unwrap();
            let conn_path = std::path::Path::new(conn);
            if !conn_path.exists() {
                let parent_dir = conn_path.parent().unwrap();
                std::fs::create_dir_all(parent_dir).unwrap();
                std::fs::File::create(conn).unwrap();
            }

            get_sqlite_conn(conn, max_conn).await?
        }
        _ => unimplemented!("This connection string is not supported: `{}`", conn_str),
    };

    Ok(conn)
}

async fn get_sqlite_conn(conn_str: &str, max_conn: u32) -> Result<Pool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(conn_str)
        .await
}

pub async fn apply_migrations(conn: &Pool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations/sqlite/").run(conn).await
}
