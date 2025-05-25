use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::fs;
use std::path::Path;


fn parse_date(date_str: &str) -> Option<DateTime<Utc>> {
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    let naive_dt = naive_date.and_hms_opt(0, 0, 0)?;
    Some(Utc.from_utc_datetime(&naive_dt))
}

pub fn is_after(path: &Path, after: &str) -> bool {
    let after_time = match parse_date(after){
        Some(d) => d,
        None => return false,
    };
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };
    let modified = match metadata.modified() {
        Ok(m) => m,
        Err(_) => return false
    };
    let file_time = DateTime::<Utc>::from(modified);
    file_time > after_time
}

pub fn is_before(path: &Path, before: &str) -> bool {
    let before_time = match parse_date(before){
        Some(d) => d,
        None => return false,
    };
    let metadata = match fs::metadata(path){
        Ok(m) => m,
        Err(_) => return false
    };
    let modified = match metadata.modified() {
        Ok(m) => m,
        Err(_) => return false
    };
    let file_time = DateTime::<Utc>::from(modified);
    file_time < before_time

}