use super::pool::DBAccessor;
use regex::Regex;
use tokio::time::Duration;

pub const DB_QUERY_TIMEOUT: Duration = Duration::from_secs(5);
pub const ASC: &str = "ASC";
pub const DESC: &str = "DESC";

pub fn detect_sql_injection(input: &str) -> bool {
    let sql_injection_pattern = Regex::new(r"(?i)(\b(?:select|insert|update|delete|drop|alter|create)\b.*\b(?:from|into|table|where)\b|\bunion\b.*\b(?:select|values)\b|\b(?:exec|execute)\b\s*\()").unwrap();
    sql_injection_pattern.is_match(input)
}

pub fn sort_direction(descending: bool) -> String {
    if descending {
        DESC.to_string()
    } else {
        ASC.to_string()
    }
}

pub fn default_sort_direction() -> String {
    ASC.to_string()
}
