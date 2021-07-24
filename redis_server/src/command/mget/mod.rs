use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandMGet {
    keys: Vec<String>,
}

impl RedisCommandMGet {
    pub fn new(keys: Vec<String>) -> RedisCommandMGet {
        RedisCommandMGet { keys }
    }
}

impl RedisCommand for RedisCommandMGet {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::MGet(self.keys.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        Ok(response)
    }
}
