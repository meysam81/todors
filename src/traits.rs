use crate::errors::TodoErrors;
use crate::serializers::{Deserialize, Serialize};
pub use async_trait::async_trait;

#[derive(Debug, Deserialize)]
pub struct ListRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
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

#[async_trait(?Send)]
pub trait Controller {
    type Id: Serialize + for<'a> Deserialize<'a>;
    type Input: for<'a> Deserialize<'a>;
    type OptionalInput: for<'a> Deserialize<'a>;
    type Output: Serialize;

    async fn create(&self, todo: Self::Input) -> Result<Self::Output, TodoErrors>;
    async fn create_batch(&self, todos: Vec<Self::Input>) -> Result<Vec<Self::Id>, TodoErrors>;
    async fn delete(&self, id: Self::Id) -> Result<(), TodoErrors>;
    async fn get(&self, id: Self::Id) -> Result<Self::Output, TodoErrors>;
    async fn list(&self, req: ListRequest) -> Result<ListResponse<Self::Output>, TodoErrors>;
    async fn update(&self, id: Self::Id, todo: Self::OptionalInput) -> Result<(), TodoErrors>;
}
