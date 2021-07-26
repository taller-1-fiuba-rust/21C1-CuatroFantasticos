use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///This command copies the value stored at the source key to the destination key
/// By default, the destination key is created in the logical database used by the connection.
/// The DB option allows specifying an alternative logical database index for the destination key.
/// The command returns an error when the destination key already exists.
/// The REPLACE option removes the destination key before copying the value to it.
///
/// # Arguments
///  * source_key - String,
///  * destination_key - String,
///
/// # Return value
///Integer reply, specifically:
/// * 1 if source was copied.
/// * 0 if source was not copied.

pub struct RedisCommandCopy {
    source_key: String,
    destination_key: String,
}

impl RedisCommandCopy {
    pub fn new(source_key: String, destination_key: String) -> RedisCommandCopy {
        RedisCommandCopy {
            source_key,
            destination_key,
        }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Copy with source_key: {} and destination_key: {}",
            self.source_key, self.destination_key
        ));
        let copy = global_resources
            .get_storage_accessor()
            .access(StorageAction::Copy(
                self.source_key.clone(),
                self.destination_key.clone(),
            ))?;
        let response = copy.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Copy");
        Ok(response)
    }
}
