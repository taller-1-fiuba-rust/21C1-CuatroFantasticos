use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSave {}

impl RedisCommandSave {
    pub fn new() -> RedisCommandSave {
        RedisCommandSave {}
    }
}

impl RedisCommand for RedisCommandSave {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Save)?;
        let response = response.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}

impl Default for RedisCommandSave {
    fn default() -> Self {
        Self::new()
    }
}
