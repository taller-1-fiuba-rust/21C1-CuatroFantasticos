use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
/// Saves an element in Storage

pub struct RedisCommandSave {}

impl RedisCommandSave {
    pub fn new() -> RedisCommandSave {
        RedisCommandSave {}
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
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
