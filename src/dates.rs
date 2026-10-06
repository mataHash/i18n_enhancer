use chrono::{DateTime, Local};
pub fn parse_dates(format: &str) -> String {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format(format);
    let str = format!("{}", custom_format);
    str
}
