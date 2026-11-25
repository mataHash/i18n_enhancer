use jiff::Timestamp;
pub fn parse_dates(format: &str) -> String {
    let time = Timestamp::now();
    let ftime: String = time.strftime(format).to_string();
    ftime
}
