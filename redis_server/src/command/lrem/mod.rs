use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandLRem {
    key: String,
    count: String,
    element: String,
}

impl RedisCommandLRem {
    pub fn new(key: String, count: String, element: String) -> RedisCommandLRem {
        RedisCommandLRem {
            key,
            count,
            element,
        }
    }

    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command LRem with key : {}, count: {} and element: {}",
            self.key, self.count, self.element
        ));
        let response = match self.count.parse::<i32>() {
            Ok(count) => {
                let response =
                    global_resources
                        .get_storage_accessor()
                        .access(StorageAction::LRem(
                            self.key.clone(),
                            count,
                            self.element.clone(),
                        ))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => {
                verbose.print("Value of argument count was not a number");
                StorageResult::Error(RedisError::NotANumber).protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command LRem");
        Ok(response)
    }
}
