use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandScard {
    key: String,
}

impl RedisCommandScard {
    pub fn new(key: String) -> RedisCommandScard {
        RedisCommandScard { key }
    }
}

impl RedisCommand for RedisCommandScard {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Scard(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
