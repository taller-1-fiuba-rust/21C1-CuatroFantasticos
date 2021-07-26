use crate::data::redis_value::RedisValue;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::global_resources::GlobalResources;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSmembers {
    key: String,
}

impl RedisCommandSmembers {
    pub fn new(key: String) -> RedisCommandSmembers {
        RedisCommandSmembers { key }
    }
    pub fn execute(&self, global_resources: GlobalResources) -> Result<String, String> {
        let verbose = global_resources.get_verbose();
        verbose.print(&format!(
            "Executing command Smembers with key: {}",
            self.key
        ));

        let response = global_resources
            .get_storage_accessor()
            .access(StorageAction::Smembers(self.key.clone()))?;
        verbose.print("Finalizing execution of command Sismember");
        let response = match response.get_value() {
            StorageResult::RedisValue(RedisValue::Set(value)) => {
                verbose.print("Got a redisValue Set response");
                value.keys().protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(RedisError::NonExistent) => {
                verbose.print("The key did not exist");
                let vec: Vec<String> = vec![];
                vec.protocol_serialize_to_bulk_string()
            }

            StorageResult::Error(value) => {
                verbose.print("The key did not store a Set");
                value.protocol_serialize_to_bulk_string()
            }
            _ => {
                verbose.print("Unexpected Error");
                RedisError::Unknown.protocol_serialize_to_bulk_string()
            }
        };
        verbose.print("Finalizing execution command Smembers");
        Ok(response)
    }
}
