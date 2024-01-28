use rbatis::rbdc::DateTime;

/// 获取当前时间
pub fn get_now_date() -> String {
    DateTime::now().format("YYYY-MM-DD hh:mm:ss")
}
