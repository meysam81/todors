use crate::consts::DEFAULT_PAGE_LIMIT;
use crate::serializers::{Deserialize, Serialize};

mod todo;
pub use todo::*;

#[derive(Debug, Deserialize)]
pub struct ListRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
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
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
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
