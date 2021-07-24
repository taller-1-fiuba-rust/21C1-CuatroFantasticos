use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

///Remove the existing timeout on key, turning the key from volatile
/// (a key with an expire set) to persistent
/// (a key that will never expire as no timeout is associated).
///
/// # Arguments
/// * key - String
///
/// # Return value
/// Integer reply, specifically:
///1 if the timeout was removed.
///0 if key does not exist or does not have an associated timeout.

pub struct RedisCommandPersist {
    key: String,
}

impl RedisCommandPersist {
    pub fn new(key: String) -> RedisCommandPersist {
        RedisCommandPersist { key }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Persist(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
