use crate::command::RedisCommand;
use crate::data::redis_value::RedisValue;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::data::storage::service::operator::response_message::StorageResult;
use crate::data::storage::service::operator::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSort {
    key: String,
}

impl RedisCommandSort {
    pub fn new(key: String) -> RedisCommandSort {
        RedisCommandSort { key }
    }
}

impl RedisCommand for RedisCommandSort {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Get(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(RedisValue::Set(value)) => match value.sort() {
                Ok(value) => value.protocol_serialize_to_bulk_string(),
                Err(value) => value.protocol_serialize_to_bulk_string(),
            },
            StorageResult::RedisValue(RedisValue::List(value)) => match value.sort() {
                Ok(value) => value.protocol_serialize_to_bulk_string(),
                Err(value) => value.protocol_serialize_to_bulk_string(),
            },
            StorageResult::RedisValue(RedisValue::String(_)) => {
                RedisError::NotAListNorSet.protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(RedisError::NonExistent) => {
                RedisError::NilArray.protocol_serialize_to_bulk_string()
            }
            StorageResult::Error(value) => value.protocol_serialize_to_bulk_string(),
            _ => StorageResult::Error(RedisError::Unknown).protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
