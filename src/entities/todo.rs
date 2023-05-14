use crate::apidoc::ToSchema;
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
