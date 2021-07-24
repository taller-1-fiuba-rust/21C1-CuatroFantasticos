use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
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
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response =
            accessor.access(StorageAction::SAdd(self.key.clone(), self.members.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
