use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::response_message::StorageResult;
use crate::data::storage_service::operator_service::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandKeys {
    pattern: String,
}

impl RedisCommandKeys {
    pub fn new(key: String) -> RedisCommandKeys {
        RedisCommandKeys { pattern: key }
    }
}

impl RedisCommand for RedisCommandKeys {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Keys(self.pattern.clone()))?;
        let response = match response.get_value() {
            StorageResult::Vector(vec) => vec.protocol_serialize_to_simple_string(),
            _ => RedisError::Unknown.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}
