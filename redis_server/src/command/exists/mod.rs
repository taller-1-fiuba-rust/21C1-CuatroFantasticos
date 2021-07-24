use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Returns if key exists.
///Since Redis 3.0.3 it is possible to specify multiple keys instead of a single one.
/// In such a case, it returns the total number of keys existing. Note that returning 1 or 0
/// for a single key is just a special case of the variadic usage,
/// so the command is completely backward compatible.
///The user should be aware that if the same existing key is mentioned in the arguments multiple times,
/// it will be counted multiple times. So if somekey exists, EXISTS somekey somekey will return 2.
///
/// # Arguments
/// * key - String
///
/// # Return value
///Integer reply, specifically:
/// * 1 if the key exists.
/// * 0 if the key does not exist.

pub struct RedisCommandExists {
    key: String,
}

impl RedisCommandExists {
    pub fn new(key: String) -> RedisCommandExists {
        RedisCommandExists { key }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Exists(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
