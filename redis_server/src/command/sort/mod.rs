use crate::command::RedisCommand;
use crate::data::redis_value::RedisValue;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageRequestMessageEnum;
use crate::data::storage_service::operator_service::response_error_enum::RedisErrorEnum;
use crate::data::storage_service::operator_service::response_message::StorageResponseMessageEnum;
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
        let response = accessor.access(StorageRequestMessageEnum::Sort(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResponseMessageEnum::RedisValue(RedisValue::Set(value)) => match value.sort() {
                Ok(value) => value.protocol_serialize_to_bulk_string(),
                Err(value) => value.protocol_serialize_to_bulk_string(),
            },
            StorageResponseMessageEnum::RedisValue(RedisValue::List(value)) => match value.sort() {
                Ok(value) => value.protocol_serialize_to_bulk_string(),
                Err(value) => value.protocol_serialize_to_bulk_string(),
            },
            StorageResponseMessageEnum::Error(value) => value.protocol_serialize_to_bulk_string(),
            _ => StorageResponseMessageEnum::Error(RedisErrorEnum::Unknown)
                .protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
