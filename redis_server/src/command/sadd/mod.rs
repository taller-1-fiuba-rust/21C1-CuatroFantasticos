use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSAdd {
    key: String,
    members: Vec<String>,
}

impl RedisCommandSAdd {
    pub fn new(key: String, members: Vec<String>) -> RedisCommandSAdd {
        RedisCommandSAdd { key, members }
    }
}

impl RedisCommand for RedisCommandSAdd {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response =
            accessor.access(StorageAction::SAdd(self.key.clone(), self.members.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        Ok(response)
    }
}
