use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

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
        let response = match response.get_value() {
            StorageResponseMessageEnum::Int(value) => value.protocol_serialize_to_int(),
            value => value.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}
