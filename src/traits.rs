use crate::errors::TodoErrors;
use crate::serializers::{Deserialize, Serialize};
pub use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller {
    type Input: for<'a> Deserialize<'a>;
    type Output: Serialize;

    async fn list(&self) -> Result<Vec<Self::Output>, TodoErrors>;
    async fn create(&self, todo: &Self::Input) -> Result<Self::Output, TodoErrors>;
}
