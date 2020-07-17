use chrono::prelude::*;

pub const SUCCESS: &'static str = "success";
pub const FAIL: &'static str = "fail";

pub const DB_TOPICS: &'static str = "db_topics";
pub const SUFFIX_CONF: &'static str = "_conf";
pub const SUFFIX_RESP: &'static str = "_resp";

pub const ENV_SLED: &'static str = "leader_url";
pub const ENV_URL: &'static str = "leader_url";

pub fn get_dot_env(name: &str) -> String {
    dotenv::dotenv().ok();
    if let Ok(v) = std::env::var(name) {
        return v;
    }
    panic!("!!!!!!!!!!no env var: {}", name);
}

pub fn time_now_str() -> String {
    let local = Local::now();
    let s = local.to_rfc3339();
    s
}

pub fn time_differ(origin: &str) -> i64 {
    let a = Local::now().time();
    let b = DateTime::parse_from_rfc3339(origin).unwrap();
    let c = b.time();
    let differ = a - c;
    let res = differ.num_seconds();
    res
}

pub fn default_string() -> String {
    "".to_string()
}
pub fn default_bool_false() -> bool {
    false
}
pub fn default_i64() -> i64 {
    10
}
pub fn default_vec_string() -> Vec<String> {
    vec![]
}

pub fn create_dir_if_not_exists(dir: &str) {
    let p = std::path::Path::new(dir);
    if p.exists() {
        return;
    };
    std::fs::create_dir(p).unwrap();
}
