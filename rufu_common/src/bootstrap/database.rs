use crate::bootstrap::application::APP_CONFIG;
use crate::errors::AppError;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use tokio::sync::OnceCell;

pub static DB: OnceCell<RBatis> = OnceCell::const_new();

/// 连接数据库: rbatis
pub async fn connect_db() {
    DB.get_or_init(|| async {
        let db_url = APP_CONFIG.database_url.to_string();
        let rb = RBatis::new();
        rb.init(MysqlDriver {}, &db_url).unwrap();

        // 初始化数据库
        init_db(&rb).await;

        return rb;
    })
    .await;
}

/// 初始化数据库
pub async fn init_db(rb: &RBatis) {
    let sql_file = match rb.driver_type().unwrap() {
        "sqlite" => "./sql/init_sqlite.sql",
        "postgres" => "./sql/init_postgres.sql",
        "mysql" => "./sql/init_mysql.sql",
        "mssql" => "./sql/init_mssql.sql",
        _ => "",
    };
    if sql_file != "" {
        let sql = std::fs::read_to_string(sql_file).unwrap();
        let _ = rb.exec(&sql, vec![]).await;
    }
}

/// 更新数据库
pub async fn update_db(rb: &RBatis) {
    let sql_file = match rb.driver_type().unwrap() {
        "sqlite" => "./sql/update_sqlite.sql",
        "postgres" => "./sql/update_postgres.sql",
        "mysql" => "./sql/update_mysql.sql",
        "mssql" => "./sql/update_mssql.sql",
        _ => "",
    };
    if sql_file != "" {
        let sql = std::fs::read_to_string(sql_file).unwrap();
        let _ = rb.exec(&sql, vec![]).await;
    }
}

/// 获取数据库实例
pub fn get_db<'a>() -> Result<&'a RBatis, AppError> {
    DB.get()
        .ok_or(AppError::RBATIS_ERROR("数据库连接失败".to_string()))
}
