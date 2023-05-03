use crate::apidoc::ToSchema;
use crate::db::{query, query_as, FromRow, Pool, Row};
use crate::entities::{ListRequest, ListResponse};
use crate::errors::TodoErrors;
use crate::serializers::{Deserialize, Serialize};
use crate::traits::{async_trait, Controller};
use std::cmp;

pub type Id = u32;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TodoRead {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TodoWrite {
    pub title: String,
    pub done: bool,
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
    pub title: Option<String>,
    pub done: Option<bool>,
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
    create_batch_hard_limit: u32,
}

impl TodoController {
    pub fn new(
        pool: Pool,
        pagination_limit: Option<u32>,
        pagination_hard_limit: Option<u32>,
        create_batch_hard_limit: Option<u32>,
    ) -> TodoController {
        TodoController {
            pool,
            pagination_limit: pagination_limit.unwrap_or(100),
            pagination_hard_limit: pagination_hard_limit.unwrap_or(1000),
            create_batch_hard_limit: create_batch_hard_limit.unwrap_or(1000),
        }
    }
}

#[async_trait]
impl Controller for TodoController {
    type Id = super::Id;
    type Input = TodoWrite;
    type OptionalInput = TodoUpdate;
    type Output = TodoRead;

    async fn create(&self, todo: Self::Input) -> Result<Self::Output, TodoErrors> {
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

    async fn create_batch(&self, todos: Vec<Self::Input>) -> Result<Vec<Self::Id>, TodoErrors> {
        if todos.len() > self.create_batch_hard_limit as usize {
            return Err(TodoErrors::BatchTooLarge {
                max_size: self.create_batch_hard_limit,
            });
        }

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

    async fn update(&self, id: Self::Id, todo: Self::OptionalInput) -> Result<(), TodoErrors> {
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
