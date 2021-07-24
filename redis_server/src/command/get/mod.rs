use crate::command::RedisCommand;
use crate::data::redis_value::RedisValue;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandGet {
    key: String,
}

impl RedisCommandGet {
    pub fn new(key: String) -> RedisCommandGet {
        RedisCommandGet { key }
    }
}

impl RedisCommand for RedisCommandGet {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Get(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(RedisValue::String(value)) => {
                value.protocol_serialize_to_bulk_string()
            }
            StorageResult::RedisValue(_) => {
                RedisError::NotAString.protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(value) => value.protocol_serialize_to_bulk_string(),
            _ => RedisError::Unknown.protocol_serialize_to_bulk_string(),
        };

        Ok(response)
    }
}
