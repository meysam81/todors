use crate::entities::{ListRequest, ListResponse};
use crate::errors::TodoErrors;
use crate::serializers::{Deserialize, Serialize};
pub use async_trait::async_trait;

#[async_trait]
pub trait Controller: Sync + Send + 'static {
    type Id: Serialize + for<'a> Deserialize<'a>;
    type Input: for<'a> Deserialize<'a>;
    type OptionalInput: for<'a> Deserialize<'a>;
    type Output: Serialize;

    async fn create(&self, todo: Self::Input) -> Result<Self::Output, TodoErrors>;
    async fn create_batch(&self, todos: Vec<Self::Input>) -> Result<Vec<Self::Output>, TodoErrors>;
    async fn delete(&self, id: Self::Id) -> Result<(), TodoErrors>;
    async fn get(&self, id: Self::Id) -> Result<Self::Output, TodoErrors>;
    async fn list(&self, req: ListRequest) -> Result<ListResponse<Self::Output>, TodoErrors>;
    async fn update(&self, id: Self::Id, todo: Self::OptionalInput) -> Result<(), TodoErrors>;
}
