use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::response_message::StorageResult;
use crate::data::storage_service::operator_service::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandTtl {
    key: String,
}

impl RedisCommandTtl {
    pub fn new(key: String) -> RedisCommandTtl {
        RedisCommandTtl { key }
    }
}

impl RedisCommand for RedisCommandTtl {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Ttl(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::Int(_) => response.get_value().protocol_serialize_to_int(),
            StorageResult::Error(RedisError::NotVolatil) => "-1".protocol_serialize_to_int(),
            StorageResult::Error(RedisError::NonExistent) => "-2".protocol_serialize_to_int(),
            _ => RedisError::Unknown.protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
