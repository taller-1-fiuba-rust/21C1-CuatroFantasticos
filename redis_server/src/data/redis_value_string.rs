use crate::data::redis_value::RedisValue;

pub struct RedisValueString {
    contents: String,
}

impl RedisValue for RedisValueString {
    fn serialize(&self) -> String {
        self.contents.clone()
    }
}

impl RedisValueString {
    pub fn new(contents: String) -> RedisValueString {
        RedisValueString { contents }
    }
}
