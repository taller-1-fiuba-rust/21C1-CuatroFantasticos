use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLPop {
    key: String,
    times: String,
}

impl RedisCommandLPop {
    pub fn new(key: String, times: String) -> RedisCommandLPop {
        RedisCommandLPop { key, times }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command LPop with key: {} and times: {}",
            self.key, self.times
        ));
        let times = self.times.parse::<i32>();
        let response = match times {
            Ok(value) => {
                verbose.print(&format!("The argument was a number with value: {}", value));
                let response = global_resources
                    .get_storage_accessor()
                    .access(StorageAction::LPop(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_simple_string()
            }
            Err(_) => {
                verbose.print("The argument was not a number");
                RedisError::NotANumber.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of commmand LPop");
        Ok(response)
    }
}
