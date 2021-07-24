use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandStrlen {
    key: String,
}

impl RedisCommandStrlen {
    pub fn new(key: String) -> RedisCommandStrlen {
        RedisCommandStrlen { key }
    }
}

impl RedisCommand for RedisCommandStrlen {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Strlen(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
