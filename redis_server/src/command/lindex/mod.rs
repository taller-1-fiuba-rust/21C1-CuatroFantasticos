use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::data::storage::response_message::StorageResponseMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLindex {
    key: String,
    index: i32,
}

impl RedisCommandLindex {
    pub fn new(key: String, index: i32) -> RedisCommandLindex {
        RedisCommandLindex { key, index }
    }
}

impl RedisCommand for RedisCommandLindex {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::Lindex(
            self.key.clone(),
            self.index,
        ))?;
        let response = match response.get_value() {
            StorageResponseMessageEnum::String(value) => value.protocol_serialize_to_bulk_string(),
            value => value.protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
