use crate::data::redis_value::RedisValue;
use crate::utilities::current_time_in_millis;
use std::fmt::Debug;

#[derive(Debug)]
pub struct StorageValue {
    redis_value: RedisValue,
    last_access_time: u128,
}

impl StorageValue {
    pub fn new(redis_value: RedisValue) -> Self {
        let current_time = current_time_in_millis();
        StorageValue {
            redis_value,
            last_access_time: current_time,
        }
    }

    pub fn last_access_time(&self) -> u128 {
        self.last_access_time
    }

    pub fn access(&mut self) -> &RedisValue {
        self.last_access_time = current_time_in_millis();
        &self.redis_value
    }

    pub fn access_mut(&mut self) -> &mut RedisValue {
        self.last_access_time = current_time_in_millis();
        &mut self.redis_value
    }

    pub fn peek(&self) -> &RedisValue {
        &self.redis_value
    }

    pub fn extract_value(self) -> RedisValue {
        self.redis_value
    }

    pub fn set_last_access_time(&mut self, value: u128) {
        self.last_access_time = value;
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
        assert_eq!(value.access().serialize(), "test".to_string());
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
