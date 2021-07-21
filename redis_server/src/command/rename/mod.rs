use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
/// Renames key to newkey. It returns an error when key does not exist.
/// If newkey already exists it is overwritten, when this happens RENAME
/// executes an implicit DEL operation, so if the deleted key contains a very big value
/// it may cause high latency even if RENAME itself is usually a constant-time operation.
///
/// # Arguments
/// key - String
/// new_key - String
///
/// # Return value
/// Simple string reply
pub struct RedisCommandRename {
    key: String,
    new_key: String,
}

impl RedisCommandRename {
    pub fn new(key: String, new_key: String) -> RedisCommandRename {
        RedisCommandRename { key, new_key }
    }
}

impl RedisCommand for RedisCommandRename {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let rename = accessor.access(StorageAction::Rename(
            self.key.clone(),
            self.new_key.clone(),
        ))?;
        let response = rename.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}
