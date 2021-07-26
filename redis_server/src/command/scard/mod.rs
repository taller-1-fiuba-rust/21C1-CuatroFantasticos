use crate::data::storage::service::operator::request_message::StorageAction;
use crate::global_resources::GlobalResources;
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
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!("Executing command Scard with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Scard(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        verbose.print("Finalizing execution of command Scard");
        Ok(response)
    }
}
