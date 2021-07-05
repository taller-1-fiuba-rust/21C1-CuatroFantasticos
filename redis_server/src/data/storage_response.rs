use crate::data::redis_value::RedisValue;

pub struct StorageResponse {
    value_response: StorageResponseEnum,
}

impl StorageResponse {
    pub fn new(value_response: StorageResponseEnum) -> StorageResponse {
        StorageResponse { value_response }
    }

    pub fn get_value(&self) -> &StorageResponseEnum {
        &self.value_response
    }
}

pub enum StorageResponseEnum {
    ResponseInt(usize),
    ResponseString(String),
    ResponseRedisValue(Box<dyn RedisValue>),
    ResponseBool(bool),
    ResponseError(String),
}
