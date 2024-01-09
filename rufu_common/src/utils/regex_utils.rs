use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    pub static ref RE_UNSIGNED_INTEGER: Regex = Regex::new(r"[1-9]\d*").unwrap();
}
