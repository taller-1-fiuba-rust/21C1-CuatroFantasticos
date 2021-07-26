use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

///Returns the element at index index in the list stored at key.
/// The index is zero-based, so 0 means the first element,
/// 1 the second element and so on. Negative indices can be used to designate
/// elements starting at the tail of the list. Here,
/// -1 means the last element, -2 means the penultimate and so forth.
///When the value at key is not a list, an error is returned.
///
/// # Arguments
/// * key - String
/// * index - String
///
/// # Return value
///Bulk string reply: the requested element, or nil when index is out of range.

pub struct RedisCommandLindex {
    key: String,
    index: String,
}

impl RedisCommandLindex {
    pub fn new(key: String, index: String) -> RedisCommandLindex {
        RedisCommandLindex { key, index }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!(
            "Executing command Lindex with key : {} and index: {}",
            self.key, self.index
        ));
        let response = match self.index.parse::<i32>() {
            Ok(index) => {
                verbose.print(&format!("Value of argument was a number: {}", index));
                let response = global_resources
                    .get_storage_accessor()
                    .access(StorageAction::Lindex(self.key.clone(), index))?;
                match response.get_value() {
                    StorageResult::String(value) => {
                        verbose.print(&format!("Got a string response: {}", value));
                        value.protocol_serialize_to_bulk_string()
                    }
                    value => {
                        verbose.print("Got an error response");
                        value.protocol_serialize_to_bulk_string()
                    }
                }
            }
            Err(_) => {
                verbose.print("Value of argument was not a number");
                StorageResult::Error(RedisError::NotANumber).protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command Lindex");
        Ok(response)
    }
}
