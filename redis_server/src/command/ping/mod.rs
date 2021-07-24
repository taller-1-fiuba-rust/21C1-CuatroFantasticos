use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::protocol_serialization::ProtocolSerializer;
/// Implemented to test the connection of the server
///
/// # Return value
/// String : PONG
pub struct RedisCommandPing {}

impl RedisCommandPing {
    pub fn new() -> RedisCommandPing {
        RedisCommandPing {}
    }
    pub fn execute(&self, _accessor: StorageAccessor) -> Result<String, String> {
        let response = "PONG".protocol_serialize_to_simple_string();
        Ok(response)
    }
}

impl Default for RedisCommandPing {
    fn default() -> Self {
        Self::new()
    }
}
