use serde::{Deserialize, Serialize};
use validator::Validate;

// 后台用户登录
#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct AdminSignRequest {
    #[validate(length(min = 4, message = "用户名最少4个字符"))]
    pub username: String,
    #[validate(length(min = 5, message = "密码最少5个字符"))]
    pub password: String,
}

// 后台用户注册
#[derive(Serialize, Deserialize, Clone, Debug, Default, Validate)]
pub struct AdminRegisterRequest {
    #[validate(length(min = 4, message = "用户名最少4个字符"))]
    pub username: String,
    #[validate(length(min = 5, message = "密码最少5个字符"))]
    pub password: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub sex: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub email: String,
}
