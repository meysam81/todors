use crate::serializers::{Deserialize, Serialize};

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

impl<T> Iterator for ListResponse<T>
where
    T: Serialize + Send + Sync + 'static,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}
