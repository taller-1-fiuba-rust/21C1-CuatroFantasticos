use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageRequestMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandDbSize {}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize {}
    }
}

impl RedisCommand for RedisCommandDbSize {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::Dbsize)?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}

impl Default for RedisCommandDbSize {
    fn default() -> Self {
        Self::new()
    }
}
