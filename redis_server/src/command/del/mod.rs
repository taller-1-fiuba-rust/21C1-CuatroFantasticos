use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Removes the specified keys. A key is ignored if it does not exist.
/// # Arguments
///  key - String
///
/// # Return value
///Integer reply: The number of keys that were removed.

pub struct RedisCommandDel {
    key: String,
}

impl RedisCommandDel {
    pub fn new(key: String) -> RedisCommandDel {
        RedisCommandDel { key }
    }
}

impl RedisCommand for RedisCommandDel {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Del(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
