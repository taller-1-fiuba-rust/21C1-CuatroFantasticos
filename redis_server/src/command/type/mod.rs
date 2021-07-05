use crate::command::RedisCommand;
use crate::data::storage_accessor::StorageAccessor;
use crate::data::storage_message::StorageMessageEnum;
use crate::data::storage_response::StorageResponseEnum;

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
        let response = accessor.access(StorageMessageEnum::Type(self.key.clone()))?;
        let value = match response.get_value(){
            StorageResponseEnum::ResponseRedisValue(value) => {
                value.as_ref().get_type()
            },
            StorageResponseEnum::ResponseError(error) => {
                error.to_owned()
            },
            _ => {
                String::from("Client did not receive a correct response from database")
            }
        };
        let response = format!("+{}\r\n", value);
        Ok(response)
    }
}
