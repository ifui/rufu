use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub server_name: String,
    // 应用程序密钥
    pub app_secret: String,
    pub debug: bool,
    pub host: String,
    pub port: u16,
    pub version: String,
    pub database_url: String,
    // JWT
    pub jwt_secret: String,
    pub jwt_expire: i64,
    // swagger ui
    pub swagger_url: String,
    pub openapi_url: String,
    // 超级管理员用户ID，多个请用英文,号分隔
    pub super_admin_id: String,
    // 角色域，多个请用英文,号分隔
    pub role_domain: String,
}
