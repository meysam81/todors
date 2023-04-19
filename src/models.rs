use serde::Deserialize;
use sqlx::{sqlite::SqliteQueryResult, Error};

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

    pub async fn save(&mut self, conn: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        let res = sqlx::query(
            r#"
            INSERT INTO todo (title, done)
            VALUES (?, ?)
            RETURNING id
            "#,
        )
        .bind(&self.title)
        .bind(self.done)
        .execute(conn)
        .await?;

        self.id = res.last_insert_rowid() as u32;

        Ok(())
    }

    pub async fn update(
        id: u32,
        title: Option<String>,
        done: Option<bool>,
        conn: &sqlx::SqlitePool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if title.is_none() && done.is_none() {
            return Err(Box::new(Error::RowNotFound));
        }

        let mut tx = conn.begin().await?;

        if let Some(title) = title {
            sqlx::query(
                r#"
                UPDATE todo
                SET title = ?
                WHERE id = ?
                "#,
            )
            .bind(title)
            .bind(id)
            .execute(&mut tx)
            .await?;
        }

        if let Some(done) = done {
            sqlx::query(
                r#"
                UPDATE todo
                SET done = ?
                WHERE id = ?
                "#,
            )
            .bind(done)
            .bind(id)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
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

    pub async fn delete(
        id: u32,
        conn: &sqlx::SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        let r = sqlx::query(
            r#"
            DELETE FROM todo
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(conn)
        .await?;

        match r.rows_affected() {
            0 => Err(Error::RowNotFound),
            _ => Ok(r),
        }
    }

    #[allow(dead_code)]
    async fn get(id: u32, conn: &sqlx::SqlitePool) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, done
            FROM todo
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(conn)
        .await?;

        Ok(todo)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{Fake, Faker};
    use sqlx::sqlite::SqlitePoolOptions;

    const TODO_TABLE_DLL: &str = r#"
    CREATE TABLE todo (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        done BOOLEAN NOT NULL
    )
    "#;

    #[tokio::test]
    async fn test_todo_create_successful() {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

        let title = Faker.fake::<String>();
        let mut todo = Todo::new(title.clone());
        todo.save(&conn).await.unwrap();

        let todos = Todo::list(&conn).await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, title);
    }

    #[tokio::test]
    async fn test_todo_update_successful() {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

        let title = Faker.fake::<String>();
        let mut todo = Todo::new(title.clone());
        todo.save(&conn).await.unwrap();

        let new_title = Faker.fake::<String>();
        Todo::update(todo.id, Some(new_title.clone()), None, &conn)
            .await
            .unwrap();

        let todo = Todo::get(todo.id, &conn).await.unwrap();
        assert_ne!(todo.title, title);
        assert_eq!(todo.title, new_title);
    }

    #[tokio::test]
    async fn test_todo_delete_successful() {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

        let title = Faker.fake::<String>();
        let mut todo = Todo::new(title.clone());
        todo.save(&conn).await.unwrap();

        let r = Todo::delete(todo.id, &conn).await.unwrap();
        assert_eq!(r.rows_affected(), 1);
    }

    #[tokio::test]
    async fn test_todo_get_successful() {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

        let title = Faker.fake::<String>();
        let mut todo = Todo::new(title.clone());
        todo.save(&conn).await.unwrap();

        let todo = Todo::get(todo.id, &conn).await.unwrap();
        assert_eq!(todo.title, title);
    }
}
