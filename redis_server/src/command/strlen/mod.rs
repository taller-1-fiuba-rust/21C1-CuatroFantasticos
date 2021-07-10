use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;

use crate::protocol_serialization::ProtocolSerializer;
use crate::data::storage::response_message::StorageResponseMessageEnum;
use crate::data::redis_value::RedisValue;

pub struct RedisCommandStrlen {
    key: String,
}

impl RedisCommandStrlen {
    pub fn new(key: String) -> RedisCommandStrlen {
        RedisCommandStrlen { key }
    }
}

impl RedisCommand for RedisCommandStrlen {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::Strlen(self.key.clone()))?;
        let response = match response.get_value(){
            StorageResponseMessageEnum::RedisValue(RedisValue::String(value)) =>{
                value.length().protocol_serialize_to_int()
            }
            StorageResponseMessageEnum::RedisValue(_value) =>{
                StorageResponseMessageEnum::Error(String::from("Value is not a String")).protocol_serialize_to_simple_string()
            }
            error => error.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}
