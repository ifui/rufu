use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PaginateRequest {
    pub page: u64,
    pub page_size: u64,
}
