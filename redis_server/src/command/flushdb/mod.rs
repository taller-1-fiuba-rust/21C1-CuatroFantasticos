use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandFlushDb {}

impl RedisCommandFlushDb {
    pub fn new() -> RedisCommandFlushDb {
        RedisCommandFlushDb {}
    }
}

impl RedisCommand for RedisCommandFlushDb {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::FlushDb)?;
        let response = response.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}

impl Default for RedisCommandFlushDb {
    fn default() -> Self {
        Self::new()
    }
}
