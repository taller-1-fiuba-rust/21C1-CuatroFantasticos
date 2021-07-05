use crate::data::redis_value::RedisValue;

pub struct StorageResponseMessage {
    value_response: StorageResponseMessageEnum,
}

impl StorageResponseMessage {
    pub fn new(value_response: StorageResponseMessageEnum) -> StorageResponseMessage {
        StorageResponseMessage { value_response }
    }

    pub fn get_value(&self) -> &StorageResponseMessageEnum {
        &self.value_response
    }
}

pub enum StorageResponseMessageEnum {
    ResponseInt(usize),
    ResponseString(String),
    ResponseRedisValue(Box<dyn RedisValue>),
    ResponseBool(bool),
    ResponseOk,
    ResponseError(String),
}
