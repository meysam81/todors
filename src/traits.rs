use crate::errors::TodoErrors;
use crate::serializers::{Deserialize, Serialize};
pub use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller {
    type Id: Serialize + for<'a> Deserialize<'a>;
    type Input: for<'a> Deserialize<'a>;
    type Output: Serialize;

    async fn create(&self, todo: &Self::Input) -> Result<Self::Output, TodoErrors>;
    async fn delete(&self, id: Self::Id) -> Result<(), TodoErrors>;
    async fn get(&self, id: Self::Id) -> Result<Self::Output, TodoErrors>;
    async fn list(&self) -> Result<Vec<Self::Output>, TodoErrors>;
}
