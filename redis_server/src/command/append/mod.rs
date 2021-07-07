use crate::command::RedisCommand;
use crate::data::storage::accessor::StorageAccessor;
use crate::data::storage::request_message::StorageRequestMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandAppend {
    key: String,
    new_value: String,
}

impl RedisCommandAppend {
    pub fn new(key: String, new_value: String) -> RedisCommandAppend {
        RedisCommandAppend { key, new_value }
    }
}

impl RedisCommand for RedisCommandAppend {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let append = accessor.access(StorageRequestMessageEnum::Append(
            self.key.clone(),
            self.new_value.clone(),
        ))?;
        let response = append.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}
