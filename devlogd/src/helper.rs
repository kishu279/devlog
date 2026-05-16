use chrono::{DateTime, Utc};

pub fn convert_datetime_to_i64(input: &str) -> i64 {
    let format_pattern = "%Y-%m-%d %H:%M:%S %z";

    DateTime::parse_from_str(input.trim(), format_pattern)
        .map(|datetime| datetime.timestamp())
        .unwrap_or_else(|_| Utc::now().timestamp())
}
