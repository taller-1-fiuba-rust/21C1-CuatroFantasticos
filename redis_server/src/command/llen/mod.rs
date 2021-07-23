use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Returns the length of the list stored at key.
/// If key does not exist, it is interpreted as an empty list and 0 is returned.
/// An error is returned when the value stored at key is not a list.
///
/// # Arguments
/// * key - String
///
/// # Return value
///Integer reply: the length of the list at key.

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
