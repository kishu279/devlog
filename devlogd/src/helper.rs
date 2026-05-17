use chrono::{DateTime, Utc};

pub fn convert_datetime_to_i64(input: &str) -> i64 {
    let format_pattern = "%Y-%m-%d %H:%M:%S %z";

    match DateTime::parse_from_str(input.trim(), format_pattern) {
        Ok(datetime) => datetime.timestamp(),

        Err(err) => {
            panic!("failed to parse git datetime: '{}'\nerror: {}", input, err);
        }
    }
}
use sha2::{Digest, Sha256};

pub fn hash_path(path: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(path.as_bytes());

    hex::encode(hasher.finalize())
}
