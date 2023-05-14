use crate::apidoc::ToSchema;
use crate::consts::DEFAULT_PAGE_LIMIT;
use crate::db::FromRow;
use crate::serializers::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
pub type Id = u32;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TodoRead {
    pub id: Id,
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

#[derive(Debug, Deserialize)]
pub struct ListRequest {
    pub limit: Option<Id>,
    pub offset: Option<Id>,
}

impl Default for ListRequest {
    fn default() -> Self {
        Self {
            limit: Some(DEFAULT_PAGE_LIMIT),
            offset: Some(0),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListResponse<T>
where
    T: Serialize,
{
    pub data: Vec<T>,
    pub total: Id,
    pub limit: Id,
    pub offset: Id,
}

impl<T> Iterator for ListResponse<T>
where
    T: Serialize + Send + Sync + 'static,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}
