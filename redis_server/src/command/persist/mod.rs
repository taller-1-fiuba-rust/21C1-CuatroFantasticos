use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandPersist {
    key: String,
}

impl RedisCommandPersist {
    pub fn new(key: String) -> RedisCommandPersist {
        RedisCommandPersist { key }
    }
}

impl RedisCommand for RedisCommandPersist {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Persist(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
