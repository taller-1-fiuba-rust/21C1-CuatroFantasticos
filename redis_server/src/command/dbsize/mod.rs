use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Return the number of keys in the currently-selected database.
/// # Return value
///* Integer reply

pub struct RedisCommandDbSize {}

impl RedisCommandDbSize {
    pub fn new() -> RedisCommandDbSize {
        RedisCommandDbSize {}
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Dbsize)?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}

impl Default for RedisCommandDbSize {
    fn default() -> Self {
        Self::new()
    }
}
