use crate::config::app_config::AppConfig;
use config::{Config, Environment, File};
use dotenv_rs::dotenv_with_prefix;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref APP_CONFIG: AppConfig = {
        let config_builder = Config::builder()
            .add_source(File::with_name("Config"))
            // 设置环境变量前缀 注意是__双下划线
            .add_source(Environment::with_prefix("RF").separator("__").try_parsing(true))
            .build()
            .expect("读取 Config.toml 配置文件失败");

        config_builder
            .try_deserialize()
            .expect("读取 Config.toml 配置文件失败")
    };
}

pub async fn start() {
    // 加载环境变量
    dotenv_with_prefix("RF").ok();

    fast_log::init(fast_log::Config::new().console()).expect("- FastLog: 日志服务启动失败");

    println!("- Local: http://{}:{}", APP_CONFIG.host, APP_CONFIG.port);
    println!("- Version: {}", APP_CONFIG.version);
    print_banner();

    // 连接数据库
    crate::bootstrap::database::connect_db().await;
}

fn print_banner() {
    let banner = r#"
 ______    __  __   ______   __  __      
/_____/\  /_/\/_/\ /_____/\ /_/\/_/\     
\:::_ \ \ \:\ \:\ \\::::_\/_\:\ \:\ \    
 \:(_) ) )_\:\ \:\ \\:\/___/\\:\ \:\ \   
  \: __ `\ \\:\ \:\ \\:::._\/ \:\ \:\ \  
   \ \ `\ \ \\:\_\:\ \\:\ \    \:\_\:\ \ 
    \_\/ \_\/ \_____\/ \_\/     \_____\/ 
"#;
    println!("{}", banner);
}
