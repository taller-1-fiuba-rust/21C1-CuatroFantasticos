use crate::configuration::service::request_message::ConfAction;
use crate::configuration::service::response_message::ConfResult;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandConfigGet {
    key: String,
}

impl RedisCommandConfigGet {
    pub fn new(key: String) -> RedisCommandConfigGet {
        RedisCommandConfigGet { key }
    }

    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command ConfigGet with key: {}",
            self.key
        ));

        let response = global_resources
            .get_configuration_accessor()
            .access(ConfAction::GetParameters(self.key.clone()))?;

        let response = match response {
            ConfResult::Vector(value) => {
                verbose.print("Received parameter successfully");
                value.protocol_serialize_to_bulk_string()
            }
            _ => {
                verbose.print("Error, Parameter did not exist");
                let vec: Vec<String> = vec![];
                vec.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of configGet command");
        Ok(response)
    }
}
