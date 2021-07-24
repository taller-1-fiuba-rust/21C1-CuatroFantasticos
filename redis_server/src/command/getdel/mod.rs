use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Get the value of key and delete the key. This command is similar to GET,
/// except for the fact that it also deletes the key on success (if and only if the key's value type is a string).
///
/// # Arguments
/// * key - String
///
/// # Return value
///Bulk string reply: the value of key, nil when key does not exist,
/// or an error if the key's value type isn't a string.

pub struct RedisCommandGetDel {
    key: String,
}

impl RedisCommandGetDel {
    pub fn new(key: String) -> RedisCommandGetDel {
        RedisCommandGetDel { key }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::GetDel(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        Ok(response)
    }
}
