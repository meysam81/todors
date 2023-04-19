use serde::Deserialize;
use sqlx::sqlite::SqliteQueryResult;

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct Todo {
    id: u32,
    pub title: String,
    done: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: 0,
            title,
            done: false,
        }
    }

    pub fn done(&mut self) {
        self.done = true;
    }

    pub async fn save(&self, conn: &sqlx::SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO todo (title, done)
            VALUES (?, ?)
            RETURNING id
            "#,
        )
        .bind(&self.title)
        .bind(self.done)
        .execute(conn)
        .await
    }

    pub async fn update(&self, conn: &sqlx::SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE todo
            SET title = ?, done = ?
            WHERE id = ?
            RETURNING id
            "#,
        )
        .bind(&self.title)
        .bind(self.done)
        .bind(self.id)
        .execute(conn)
        .await
    }

    pub async fn list(conn: &sqlx::SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, done
            FROM todo
            "#,
        )
        .fetch_all(conn)
        .await?;

        Ok(todos)
    }
}
