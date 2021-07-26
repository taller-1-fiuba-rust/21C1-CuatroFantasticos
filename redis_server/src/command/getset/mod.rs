use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Atomically sets key to value and returns the old value stored at key.
/// Returns an error when key exists but does not hold a string value.
/// Any previous time to live associated with the key is discarded on successful SET operation.
///
/// # Arguments
/// * key - String
/// * new_value - String
///
/// # Return value
///Bulk string reply: the old value stored at key, or nil when key did not exist.
pub struct RedisCommandGetSet {
    key: String,
    new_value: String,
}

impl RedisCommandGetSet {
    pub fn new(key: String, new_value: String) -> RedisCommandGetSet {
        RedisCommandGetSet { key, new_value }
    }

    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command GetSet with key: {} and value: {}",
            self.key, self.new_value
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::GetSet(
                self.key.clone(),
                self.new_value.clone(),
            ))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        verbose.print("Finalizing execution of command GetSet");
        Ok(response)
    }
}
