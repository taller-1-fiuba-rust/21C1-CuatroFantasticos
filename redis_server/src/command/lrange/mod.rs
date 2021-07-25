use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLRange {
    key: String,
    start: String,
    stop: String,
}

impl RedisCommandLRange {
    pub fn new(key: String, start: String, stop: String) -> RedisCommandLRange {
        RedisCommandLRange { key, start, stop }
    }

    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command LRange with key : {}, start: {} and stop: {}",
            self.key, self.start, self.stop
        ));
        let response = match self.start.parse::<i32>() {
            Ok(start) => match self.stop.parse::<i32>() {
                Ok(stop) => {
                    let response = global_resources
                        .get_storage_accessor()
                        .access(StorageAction::LRange(self.key.clone(), start, stop))?;
                    response.get_value().protocol_serialize_to_bulk_string()
                }
                Err(_) => {
                    verbose.print("Value of argument stop was not a number");
                    StorageResult::Error(RedisError::NotANumber).protocol_serialize_to_bulk_string()
                }
            },
            Err(_) => {
                verbose.print("Value of argument start was not a number");
                StorageResult::Error(RedisError::NotANumber).protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command LRange");
        Ok(response)
    }
}
