use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Alters the last access time of a key(s). A key is ignored if it does not exist.
///
/// # Arguments
/// key - String
///
/// # Return value
/// Integer reply: The number of keys that were touched.
pub struct RedisCommandTouch {
    key: String,
}

impl RedisCommandTouch {
    pub fn new(key: String) -> RedisCommandTouch {
        RedisCommandTouch { key }
    }
}

impl RedisCommand for RedisCommandTouch {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Touch(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
