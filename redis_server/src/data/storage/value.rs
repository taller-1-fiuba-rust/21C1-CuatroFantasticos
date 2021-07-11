use crate::data::redis_value::RedisValue;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StorageValue {
    redis_value: RedisValue,
    expiration_time: Option<u128>,
    last_access_time: u128,
}

fn current_time_in_millis() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(value) => value.as_millis(),
        Err(_) => 0,
    }
}

impl StorageValue {
    pub fn new(redis_value: RedisValue) -> Self {
        let current_time = current_time_in_millis();
        StorageValue {
            redis_value,
            expiration_time: None,
            last_access_time: current_time,
        }
    }

    pub fn set_expiration_time(&mut self, expiration_time: u128) {
        self.expiration_time = Some(expiration_time);
    }

    pub fn last_access_time(&self) -> u128 {
        self.last_access_time
    }

    pub fn access(&mut self) -> Option<RedisValue> {
        if let Some(expiration_time) = self.expiration_time {
            if expiration_time < current_time_in_millis() {
                return None;
            }
        }
        self.last_access_time = current_time_in_millis();
        Some(self.redis_value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::redis_value::string::RedisValueString;
    use crate::data::redis_value::RedisValue;
    use crate::data::storage::value::StorageValue;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_access_value() {
        let redis_value = RedisValue::String(RedisValueString::new("test".to_string()));
        let mut value = StorageValue::new(redis_value);
        assert_eq!(value.access().unwrap().serialize(), "test".to_string());
    }

    #[test]
    fn test_try_access_expirated_value() {
        let redis_value = RedisValue::String(RedisValueString::new("test".to_string()));
        let mut value = StorageValue::new(redis_value);
        value.set_expiration_time(0);
        assert!(value.access().is_none());
    }

    #[test]
    fn test_last_access_time_updates_when_accessed() {
        let redis_value = RedisValue::String(RedisValueString::new("test".to_string()));
        let mut value = StorageValue::new(redis_value);
        let last_access_time_1 = value.last_access_time();
        sleep(Duration::new(0, 4000000));
        value.access();
        let last_access_time_2 = value.last_access_time();
        assert!(last_access_time_2 > last_access_time_1);
        sleep(Duration::new(0, 4000000));
        value.access();
        let last_access_time_3 = value.last_access_time();
        assert!(last_access_time_3 > last_access_time_2);
    }
}
