use crate::db::{query, query_as, Pool, Row};
use crate::entities::{Id, TodoRead, TodoUpdate, TodoWrite};
use crate::entities::{ListRequest, ListResponse};
use crate::errors::TodoErrors;
use crate::traits::{async_trait, Controller};
use std::cmp;

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
    type Id = Id;
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
            title: todo.title,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{connect, Pool};
    use fake::{Dummy, Fake, Faker};

    #[derive(Debug, Dummy, Clone)]
    struct RandomTodo {
        title: String,
        done: bool,
    }

    impl From<RandomTodo> for TodoWrite {
        fn from(todo: RandomTodo) -> Self {
            Self {
                title: todo.title,
                done: todo.done,
            }
        }
    }

    impl From<RandomTodo> for TodoUpdate {
        fn from(todo: RandomTodo) -> Self {
            Self {
                title: Some(todo.title),
                done: Some(todo.done),
            }
        }
    }

    /// get a in-memory database
    async fn get_pool() -> Pool {
        connect("sqlite://:memory:", None).await.unwrap()
    }

    fn get_rand_todo() -> RandomTodo {
        Faker.fake::<RandomTodo>()
    }

    async fn get_controller() -> TodoController {
        let pool = get_pool().await;
        TodoController::new(pool, None, None, None)
    }

    #[tokio::test]
    async fn create_result_is_the_same_as_input() {
        let controller = get_controller().await;

        let todo = get_rand_todo();

        let res = controller.create(todo.clone().into()).await.unwrap();

        assert_eq!(res.title, todo.title);
        assert_eq!(res.done, todo.done);
    }

    #[tokio::test]
    async fn create_batch_result_is_the_same_as_input() {
        let controller = get_controller().await;

        let todos = vec![get_rand_todo(), get_rand_todo(), get_rand_todo()];

        let res = controller
            .create_batch(todos.clone().into_iter().map(|t| t.into()).collect())
            .await
            .unwrap();

        assert_eq!(res.len(), todos.len());
    }

    #[tokio::test]
    async fn create_batch_more_than_hard_limit_returns_error() {
        let controller = TodoController::new(get_pool().await, None, None, Some(1));

        let todos = vec![get_rand_todo(), get_rand_todo(), get_rand_todo()];

        let res = controller
            .create_batch(todos.clone().into_iter().map(|t| t.into()).collect())
            .await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn delete_returns_error_if_todo_not_found() {
        let controller = get_controller().await;
        let random_id = Faker.fake::<u32>();

        let res = controller.delete(random_id).await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn delete_returns_ok_if_todo_found() {
        let controller = get_controller().await;

        let todo = get_rand_todo();

        let res = controller.create(todo.into()).await.unwrap();

        let res = controller.delete(res.id).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn get_returns_error_if_todo_not_found() {
        let controller = get_controller().await;
        let random_id = Faker.fake::<u32>();

        let res = controller.get(random_id).await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn get_returns_ok_if_todo_found() {
        let controller = get_controller().await;

        let todo = get_rand_todo();

        let res = controller.create(todo.clone().into()).await.unwrap();

        let res = controller.get(res.id).await.unwrap();

        assert_eq!(res.title, todo.title);
        assert_eq!(res.done, todo.done);
    }

    #[tokio::test]
    async fn list_returns_empty_with_empty_db() {
        let controller = get_controller().await;

        let res = controller.list(ListRequest::default()).await.unwrap();

        assert_eq!(res.data.len(), 0);
    }

    #[tokio::test]
    async fn list_returns_all_todos() {
        let controller = get_controller().await;

        let todos = vec![get_rand_todo(), get_rand_todo(), get_rand_todo()];

        controller
            .create_batch(todos.clone().into_iter().map(|t| t.into()).collect())
            .await
            .unwrap();

        let res = controller.list(ListRequest::default()).await.unwrap();

        assert_eq!(res.data.len(), todos.len());
    }

    #[tokio::test]
    async fn list_returns_limited_todos() {
        let batch_hard_limit = 1;
        let controller = TodoController::new(get_pool().await, None, Some(batch_hard_limit), None);

        let todos = vec![get_rand_todo(), get_rand_todo(), get_rand_todo()];

        controller
            .create_batch(todos.clone().into_iter().map(|t| t.into()).collect())
            .await
            .unwrap();

        let res = controller.list(ListRequest::default()).await.unwrap();

        assert_eq!(res.data.len(), batch_hard_limit as usize);
    }

    #[tokio::test]
    async fn update_todo() {
        let controller = get_controller().await;

        let todo = get_rand_todo();

        let res = controller.create(todo.into()).await.unwrap();

        let res = controller.update(res.id, get_rand_todo().into()).await;

        assert!(res.is_ok());
    }
}
