use crate::errors::TodoErrors;
use crate::serializers::Serialize;
pub use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller {
    type Model: Serialize;

    async fn list(self: &Self) -> Result<Vec<Self::Model>, TodoErrors>;
}
