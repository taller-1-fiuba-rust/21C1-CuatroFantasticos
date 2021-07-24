use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
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
        let response = accessor.access(StorageAction::Llen(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
