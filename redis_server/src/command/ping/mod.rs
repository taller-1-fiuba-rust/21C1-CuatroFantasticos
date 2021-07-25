use crate::global_resources::GlobalResources;
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
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print("Executing command Ping");
        let response = "PONG".protocol_serialize_to_simple_string();
        verbose.print("Finalizing execution of command Ping");
        Ok(response)
    }
}

impl Default for RedisCommandPing {
    fn default() -> Self {
        Self::new()
    }
}
