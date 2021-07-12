use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLlen {
    key: String,
}

impl RedisCommandLlen {
    pub fn new(key: String) -> RedisCommandLlen {
        RedisCommandLlen { key }
    }
}

impl RedisCommand for RedisCommandLlen {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::Llen(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResponseMessageEnum::Int(value) => value.protocol_serialize_to_int(),
            value => value.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}
