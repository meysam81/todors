use crate::consts;
use crate::db::{query, query_as, Pool};
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
            pagination_limit: pagination_limit.unwrap_or(consts::DEFAULT_PAGE_LIMIT),
            pagination_hard_limit: pagination_hard_limit.unwrap_or(consts::DEFAULT_PAGE_HARD_LIMIT),
            create_batch_hard_limit: create_batch_hard_limit
                .unwrap_or(consts::DEFAULT_CREATE_BATCH_HARD_LIMIT),
        }
    }
}

#[async_trait]
impl Controller for TodoController {
    type Id = Id;
    type Input = TodoWrite;
    type OptionalInput = TodoUpdate;
    type Output = TodoRead;

    #[inline]
    async fn create(&self, todo: Self::Input) -> Result<Self::Output, TodoErrors> {
        let res = query_as!(
            TodoRead,
            r#"
            INSERT INTO todo (title, done)
            VALUES ($1, $2)
            RETURNING id AS "id!: u32", title AS "title!", done AS "done!"
            "#,
            todo.title,
            todo.done,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(TodoRead {
            id: res.id,
            title: res.title,
            done: res.done,
        })
    }

    #[inline]
    async fn create_batch(&self, todos: Vec<Self::Input>) -> Result<Vec<Self::Output>, TodoErrors> {
        if todos.len() > self.create_batch_hard_limit as usize {
            return Err(TodoErrors::BatchTooLarge {
                max_size: self.create_batch_hard_limit,
            });
        }

        let mut tx = self.pool.begin().await?;

        let mut result = Vec::with_capacity(todos.len());

        for todo in todos {
            let res = query_as!(
                TodoRead,
                r#"
                INSERT INTO todo (title, done)
                VALUES ($1, $2)
                RETURNING id AS "id!: u32", title AS "title!", done AS "done!"
                "#,
                todo.title,
                todo.done,
            )
            .fetch_one(&mut tx)
            .await?;

            result.push(TodoRead {
                id: res.id,
                title: res.title,
                done: res.done,
            });
        }

        tx.commit().await?;

        Ok(result)
    }

    #[inline]
    async fn delete(&self, id: Self::Id) -> Result<(), TodoErrors> {
        let res = query!(
            r#"
            DELETE FROM todo
            WHERE id = $1
            RETURNING id AS "id!: u32"
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await;

        match res {
            Err(crate::db::Error::RowNotFound) => Err(TodoErrors::TodoNotFound),
            Err(_) => Err(TodoErrors::InternalError),
            _ => Ok(()),
        }
    }

    #[inline]
    async fn get(&self, id: Self::Id) -> Result<Self::Output, TodoErrors> {
        let todo = query_as!(
            TodoRead,
            r#"
            SELECT id AS "id!: u32", title, done
            FROM todo
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await;

        match todo {
            Ok(todo) => Ok(todo),
            Err(_) => Err(TodoErrors::TodoNotFound),
        }
    }

    #[inline]
    async fn list(&self, req: ListRequest) -> Result<ListResponse<Self::Output>, TodoErrors> {
        let limit = cmp::min(
            req.limit.unwrap_or(self.pagination_limit),
            self.pagination_hard_limit,
        );
        let offset = req.offset.unwrap_or(0);

        let todos = query_as!(
            TodoRead,
            r#"
            SELECT id AS "id!: u32", title, done
            FROM todo
            ORDER BY id
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await?;

        let count_query = query!(
            r#"
            SELECT COUNT(id) as "total!: u32"
            FROM todo
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let queried_count = todos.len() as u32;

        Ok(ListResponse {
            data: todos,
            total: count_query.total,
            limit: cmp::min(req.limit.unwrap_or(self.pagination_limit), queried_count),
            offset,
        })
    }

    #[inline]
    async fn update(&self, id: Self::Id, todo: Self::OptionalInput) -> Result<(), TodoErrors> {
        let mut tx = self.pool.begin().await?;

        if let Some(title) = &todo.title {
            query!(
                r#"
                UPDATE todo
                SET title = $1
                WHERE id = $2
                "#,
                title,
                id
            )
            .execute(&mut tx)
            .await?;
        }

        if let Some(done) = &todo.done {
            query!(
                r#"
                UPDATE todo
                SET done = $1
                WHERE id = $2
                "#,
                done,
                id
            )
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
