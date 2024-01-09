use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct DefaultBatchDeleteRequest {
    pub ids: Option<Vec<u32>>,
}
