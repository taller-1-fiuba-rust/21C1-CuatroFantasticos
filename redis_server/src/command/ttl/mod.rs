use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandTtl {
    key: String,
}

impl RedisCommandTtl {
    pub fn new(key: String) -> RedisCommandTtl {
        RedisCommandTtl { key }
    }
}

impl RedisCommand for RedisCommandTtl {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Ttl(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
