use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLSet {
    key: String,
    index: String,
    value: String,
}

impl RedisCommandLSet {
    pub fn new(key: String, index: String, value: String) -> RedisCommandLSet {
        RedisCommandLSet { key, index, value }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command LSet with key : {}, index: {} and value: {}",
            self.key, self.index, self.value
        ));
        let response = match self.index.parse::<i32>() {
            Ok(index) => {
                let response =
                    global_resources
                        .get_storage_accessor()
                        .access(StorageAction::LSet(
                            self.key.clone(),
                            index,
                            self.value.clone(),
                        ))?;
                response.get_value().protocol_serialize_to_bulk_string()
            }
            Err(_) => {
                verbose.print("Value of argument was not a number");
                StorageResult::Error(RedisError::NotANumber).protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command LSet");
        Ok(response)
    }
}
