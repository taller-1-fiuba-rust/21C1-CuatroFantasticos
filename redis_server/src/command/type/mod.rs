use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;

pub struct RedisCommandType {
    key: String,
}

impl RedisCommandType {
    pub fn new(key: String) -> RedisCommandType {
        RedisCommandType { key }
    }
}

impl RedisCommand for RedisCommandType {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::Type(self.key.clone()))?;
        let value = match response.get_value() {
            StorageResponseMessageEnum::ResponseRedisValue(value) => value.as_ref().get_type(),
            StorageResponseMessageEnum::ResponseError(error) => error.to_owned(),
            _ => String::from("Client did not receive a correct response from database"),
        };
        let response = format!("+{}\r\n", value);
        Ok(response)
    }
}
