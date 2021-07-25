use crate::configuration::service::request_message::ConfAction;
use crate::configuration::service::response_message::{ConfError, ConfResult};
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;
pub struct RedisCommandConfigSet {
    key: String,
    value: String,
}

impl RedisCommandConfigSet {
    pub fn new(key: String, value: String) -> RedisCommandConfigSet {
        RedisCommandConfigSet { key, value }
    }

    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command ConfigSet with key: {} and value: {}",
            self.key, self.value
        ));

        let response = global_resources
            .get_configuration_accessor()
            .access(ConfAction::Set(self.key.clone(), self.value.clone()))?;
        let result = match response {
            ConfResult::Ok => response.protocol_serialize_to_simple_string(),
            ConfResult::Error(value) => value.protocol_serialize_to_simple_string(),
            _ => ConfError::Unknown.protocol_serialize_to_simple_string(),
        };
        verbose.print("Finalizing execution of ConfigSet command");
        println!("result is: {}", result);
        Ok(result)
    }
}
