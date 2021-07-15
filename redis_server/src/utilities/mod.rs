use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_in_millis() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(value) => value.as_millis(),
        Err(_) => 0,
    }
}
