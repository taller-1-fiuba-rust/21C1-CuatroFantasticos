use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

/// Renames key to newkey. It returns an error when key does not exist.
/// If newkey already exists it is overwritten, when this happens RENAME
/// executes an implicit DEL operation, so if the deleted key contains a very big value
/// it may cause high latency even if RENAME itself is usually a constant-time operation.
///
/// # Arguments
/// * key - String
/// * new_key - String
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
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Rename with key: {} and replacing key: {} ",
            self.key, self.new_key
        ));
        let rename = global_resources
            .get_storage_accessor()
            .access(StorageAction::Rename(
                self.key.clone(),
                self.new_key.clone(),
            ))?;
        let response = rename.get_value().protocol_serialize_to_simple_string();
        verbose.print("Finalizing execution of command Rename");
        Ok(response)
    }
}
