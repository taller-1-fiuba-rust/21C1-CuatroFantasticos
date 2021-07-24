use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Removes the specified keys. A key is ignored if it does not exist.
/// # Arguments
///  * key - String
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
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Del(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
