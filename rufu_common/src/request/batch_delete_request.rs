use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct BatchDeleteRequest {
    pub ids: Option<Vec<u64>>,
}
