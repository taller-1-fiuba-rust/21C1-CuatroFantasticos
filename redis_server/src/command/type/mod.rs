use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

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
        let response = match response.get_value() {
            StorageResponseMessageEnum::RedisValue(value) => {
                StorageResponseMessageEnum::String(value.get_type())
                    .protocol_serialize_to_simple_string()
            }
            StorageResponseMessageEnum::Error(value) => {
                StorageResponseMessageEnum::Error(value.to_string())
                    .protocol_serialize_to_simple_string()
            }
            _ => "Client did not receive a correct response from the database".to_string(),
        };
        Ok(response)
    }
}
