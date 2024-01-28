use crate::bootstrap::application::APP_CONFIG;
use validator::ValidationError;

pub fn validate_domain(value: &str) -> Result<(), ValidationError> {
    let role_domain_list: &Vec<&str> = &APP_CONFIG.role_domain.split(",").collect();

    for item in role_domain_list {
        if item.to_lowercase().eq(&value.to_lowercase()) {
            return Ok(());
        }
    }

    Err(ValidationError::new("domain 参数不规范"))
}
