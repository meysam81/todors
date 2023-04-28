use crate::apidoc::ToSchema;
use crate::db::{query, query_as, FromRow, Pool, Row};
use crate::errors::TodoErrors;
use crate::serializers::{Deserialize, Serialize};
use crate::traits::{async_trait, Controller, ListRequest, ListResponse};
use std::cmp;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TodoRead {
    id: u32,
    pub title: String,
    done: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TodoWrite {
    title: String,
    done: bool,
}

impl TodoWrite {
    pub fn new(title: String, done: Option<bool>) -> Self {
        Self {
            title,
            done: done.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TodoUpdate {
    title: Option<String>,
    done: Option<bool>,
}

impl TodoUpdate {
    pub fn new(title: Option<String>, done: Option<bool>) -> Self {
        Self { title, done }
    }
}

pub struct TodoController {
    pool: Pool,
    pagination_limit: u32,
    pagination_hard_limit: u32,
}

impl TodoController {
    pub fn new(
        pool: Pool,
        pagination_limit: Option<u32>,
        pagination_hard_limit: Option<u32>,
    ) -> TodoController {
        TodoController {
            pool,
            pagination_limit: pagination_limit.unwrap_or(100),
            pagination_hard_limit: pagination_hard_limit.unwrap_or(1000),
        }
    }
}

#[async_trait(?Send)]
impl Controller for TodoController {
    type Id = u32;
    type Input = TodoWrite;
    type OptionalInput = TodoUpdate;
    type Output = TodoRead;

    async fn create(&self, todo: &Self::Input) -> Result<Self::Output, TodoErrors> {
        let res = query(
            r#"
            INSERT INTO todo (title, done)
            VALUES (?, ?)
            RETURNING id
            "#,
        )
        .bind(&todo.title)
        .bind(todo.done)
        .execute(&self.pool)
        .await?;

        let id = res.last_insert_rowid() as u32;

        Ok(TodoRead {
            id,
            title: todo.title.clone(),
            done: todo.done,
        })
    }

    async fn create_batch(&self, todos: &[Self::Input]) -> Result<Vec<Self::Id>, TodoErrors> {
        let mut tx = self.pool.begin().await?;

        let mut ids = Vec::with_capacity(todos.len());

        for todo in todos {
            let res = query(
                r#"
                INSERT INTO todo (title, done)
                VALUES (?, ?)
                RETURNING id
                "#,
            )
            .bind(&todo.title)
            .bind(todo.done)
            .execute(&mut tx)
            .await?;

            ids.push(res.last_insert_rowid() as u32);
        }

        tx.commit().await?;

        Ok(ids)
    }

    async fn delete(&self, id: Self::Id) -> Result<(), TodoErrors> {
        let r = query("DELETE FROM todo WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        match r.rows_affected() {
            0 => Err(TodoErrors::TodoNotFound),
            _ => Ok(()),
        }
    }

    async fn get(&self, id: Self::Id) -> Result<Self::Output, TodoErrors> {
        let todo = query_as::<_, TodoRead>(
            r#"
            SELECT id, title, done
            FROM todo
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await;

        match todo {
            Ok(todo) => Ok(todo),
            Err(_) => Err(TodoErrors::TodoNotFound),
        }
    }

    async fn list(&self, req: ListRequest) -> Result<ListResponse<Self::Output>, TodoErrors> {
        let limit = cmp::min(
            req.limit.unwrap_or(self.pagination_limit),
            self.pagination_hard_limit,
        );
        let todos = query_as::<_, TodoRead>(
            r#"
            SELECT id, title, done
            FROM todo
            ORDER BY id
            LIMIT ?
            OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(req.offset.unwrap_or(0))
        .fetch_all(&self.pool)
        .await?;

        let total = query(
            r#"
            SELECT COUNT(*) as total
            FROM todo
            "#,
        )
        .fetch_one(&self.pool)
        .await?
        .get::<u32, _>("total");

        let queried_count = todos.len() as u32;

        Ok(ListResponse {
            data: todos,
            total,
            limit: cmp::min(req.limit.unwrap_or(self.pagination_limit), queried_count),
            offset: req.offset.unwrap_or(0),
        })
    }

    async fn update(&self, id: Self::Id, todo: &Self::OptionalInput) -> Result<(), TodoErrors> {
        let mut tx = self.pool.begin().await?;

        if let Some(title) = &todo.title {
            query(
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

        if let Some(done) = &todo.done {
            query(
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

        // NOTE: `changes()` will always return zero if called after the tx
        // it will also return always one if called within the the tx
        // I don't know of any other way to get the number of rows affected ATM

        Ok(())
    }
}

// impl Todo {
//     pub async fn save(&mut self, conn: &Pool) -> Result<(), DbError> {
//         let res = sqlx::query(
//             r#"
//             INSERT INTO todo (title, done)
//             VALUES (?, ?)
//             RETURNING id
//             "#,
//         )
//         .bind(&self.title)
//         .bind(self.done)
//         .execute(conn)
//         .await?;

//         self.id = res.last_insert_rowid() as u32;

//         Ok(())
//     }

//     pub async fn update(
//         id: u32,
//         title: Option<String>,
//         done: Option<bool>,
//         conn: &Pool,
//     ) -> Result<(), TodoErrors> {
//         if title.is_none() && done.is_none() {
//             return Err(TodoErrors::NoUpdate);
//         }

//         let mut tx = conn.begin().await?;

//         if let Some(title) = title {
//             sqlx::query(
//                 r#"
//                 UPDATE todo
//                 SET title = ?
//                 WHERE id = ?
//                 "#,
//             )
//             .bind(title)
//             .bind(id)
//             .execute(&mut tx)
//             .await?;
//         }

//         if let Some(done) = done {
//             sqlx::query(
//                 r#"
//                 UPDATE todo
//                 SET done = ?
//                 WHERE id = ?
//                 "#,
//             )
//             .bind(done)
//             .bind(id)
//             .execute(&mut tx)
//             .await?;
//         }

//         tx.commit().await?;

//         Ok(())
//     }

//     pub async fn list(conn: &Pool) -> Result<Vec<Todo>, DbError> {
//         let todos = sqlx::query_as::<_, Todo>(
//             r#"
//             SELECT id, title, done
//             FROM todo
//             "#,
//         )
//         .fetch_all(conn)
//         .await?;

//         Ok(todos)
//     }

//     pub async fn delete(id: u32, conn: &Pool) -> Result<QueryResult, DbError> {
//         let r = sqlx::query(
//             r#"
//             DELETE FROM todo
//             WHERE id = ?
//             "#,
//         )
//         .bind(id)
//         .execute(conn)
//         .await?;

//         match r.rows_affected() {
//             0 => Err(DbError::RowNotFound),
//             _ => Ok(r),
//         }
//     }

//     pub async fn get(id: u32, conn: &Pool) -> Result<Todo, DbError> {
//         let todo = sqlx::query_as::<_, Todo>(
//             r#"
//             SELECT id, title, done
//             FROM todo
//             WHERE id = ?
//             "#,
//         )
//         .bind(id)
//         .fetch_one(conn)
//         .await?;

//         Ok(todo)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use fake::{Fake, Faker};
//     use sqlx::sqlite::SqlitePoolOptions;

//     const TODO_TABLE_DLL: &str = r#"
//     CREATE TABLE todo (
//         id INTEGER PRIMARY KEY,
//         title TEXT NOT NULL,
//         done BOOLEAN NOT NULL
//     )
//     "#;

//     #[tokio::test]
//     async fn test_todo_create_successful() {
//         let conn = SqlitePoolOptions::new()
//             .max_connections(1)
//             .connect("sqlite::memory:")
//             .await
//             .unwrap();

//         sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

//         let title = Faker.fake::<String>();
//         let mut todo = TodoRead::new(title.clone());
//         todo.save(&conn).await.unwrap();

//         let todos = TodoRead::list(&conn).await.unwrap();
//         assert_eq!(todos.len(), 1);
//         assert_eq!(todos[0].title, title);
//     }

//     #[tokio::test]
//     async fn test_todo_update_successful() {
//         let conn = SqlitePoolOptions::new()
//             .max_connections(1)
//             .connect("sqlite::memory:")
//             .await
//             .unwrap();

//         sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

//         let title = Faker.fake::<String>();
//         let mut todo = TodoRead::new(title.clone());
//         todo.save(&conn).await.unwrap();

//         let new_title = Faker.fake::<String>();
//         TodoRead::update(todo.id, Some(new_title.clone()), None, &conn)
//             .await
//             .unwrap();

//         let todo = TodoRead::get(todo.id, &conn).await.unwrap();
//         assert_ne!(todo.title, title);
//         assert_eq!(todo.title, new_title);
//     }

//     #[tokio::test]
//     async fn test_todo_delete_successful() {
//         let conn = SqlitePoolOptions::new()
//             .max_connections(1)
//             .connect("sqlite::memory:")
//             .await
//             .unwrap();

//         sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

//         let title = Faker.fake::<String>();
//         let mut todo = TodoRead::new(title.clone());
//         todo.save(&conn).await.unwrap();

//         let r = TodoRead::delete(todo.id, &conn).await.unwrap();
//         assert_eq!(r.rows_affected(), 1);
//     }

//     #[tokio::test]
//     async fn test_todo_get_successful() {
//         let conn = SqlitePoolOptions::new()
//             .max_connections(1)
//             .connect("sqlite::memory:")
//             .await
//             .unwrap();

//         sqlx::query(TODO_TABLE_DLL).execute(&conn).await.unwrap();

//         let title = Faker.fake::<String>();
//         let mut todo = TodoRead::new(title.clone());
//         todo.save(&conn).await.unwrap();

//         let todo = TodoRead::get(todo.id, &conn).await.unwrap();
//         assert_eq!(todo.title, title);
//     }
// }
