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
}
