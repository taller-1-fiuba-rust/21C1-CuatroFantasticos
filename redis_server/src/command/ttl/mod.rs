use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandTtl {
    key: String,
}

impl RedisCommandTtl {
    pub fn new(key: String) -> RedisCommandTtl {
        RedisCommandTtl { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose().expect("There is no verbose");
        verbose.print(&format!("Executing command Ttl with key: {}", self.key));
        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Ttl(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::Int(_) => {
                verbose.print("got an Int response");
                response.get_value().protocol_serialize_to_int()
            }
            StorageResult::Error(RedisError::NotVolatile) => {
                verbose.print("Key is not volatile");
                "-1".protocol_serialize_to_int()
            }
            StorageResult::Error(RedisError::NonExistent) => {
                verbose.print("Key did not exist");
                "-2".protocol_serialize_to_int()
            }
            _ => {
                verbose.print("Unexpected Error response");
                RedisError::Unknown.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution of command Ttl");
        Ok(response)
    }
}
