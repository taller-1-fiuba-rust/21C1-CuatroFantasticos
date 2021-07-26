use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Add the specified members to the set stored at key.
/// Specified members that are already a member of this set are ignored.
/// If key does not exist, a new set is created before adding the specified members.
///An error is returned when the value stored at key is not a set.
///
/// # Arguments
/// * key - String
/// * members - A Vector of Strings
///
/// # Return value
///Integer reply: the number of elements that were added to the set,
/// not including all the elements already present in the set.

pub struct RedisCommandSAdd {
    key: String,
    members: Vec<String>,
}

impl RedisCommandSAdd {
    pub fn new(key: String, members: Vec<String>) -> RedisCommandSAdd {
        RedisCommandSAdd { key, members }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command SAdd with key: {} and members: {:?} ",
            self.key, self.members
        ));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::SAdd(self.key.clone(), self.members.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command SAdd");
        Ok(response)
    }
}
