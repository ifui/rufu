use serde::{Deserialize, Serialize};
use validator::Validate;

// 后台用户登录
#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct AdminSignRequest {
    #[validate(length(min = 4, message = "用户名最少4个字符"))]
    pub username: Option<String>,
    #[validate(length(min = 5, message = "密码最少5个字符"))]
    pub password: Option<String>,
}

// 后台用户注册
#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct AdminRegisterRequest {
    #[validate(length(min = 4, message = "用户名最少4个字符"))]
    pub username: Option<String>,
    #[validate(length(min = 5, message = "密码最少5个字符"))]
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub sex: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
