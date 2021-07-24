use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;
///Returns the set cardinality (number of elements) of the set stored at key.
/// # Arguments
/// * key - String
///
///# Return value
/// * Integer reply: the cardinality (number of elements) of the set, or 0 if key does not exist.
///
///Examples
/// ```redis> SADD myset "Hello"
///(integer) 1
///redis> SADD myset "World"
///(integer) 1
///redis> SCARD myset
///(integer) 2
///redis>
/// ```
pub struct RedisCommandScard {
    key: String,
}

impl RedisCommandScard {
    pub fn new(key: String) -> RedisCommandScard {
        RedisCommandScard { key }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Scard(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
