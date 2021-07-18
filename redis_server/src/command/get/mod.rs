use crate::command::RedisCommand;
use crate::data::redis_value::RedisValue;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageRequestMessageEnum;
use crate::data::storage_service::operator_service::response_error_enum::RedisErrorEnum;
use crate::data::storage_service::operator_service::response_message::StorageResponseMessageEnum;
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
        let response = accessor.access(StorageRequestMessageEnum::Get(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResponseMessageEnum::RedisValue(RedisValue::String(value)) => {
                value.protocol_serialize_to_bulk_string()
            }
            StorageResponseMessageEnum::RedisValue(_) => {
                RedisErrorEnum::NotAString.protocol_serialize_to_bulk_string()
            }
            StorageResponseMessageEnum::Error(value) => value.protocol_serialize_to_bulk_string(),
            _ => RedisErrorEnum::Unknown.protocol_serialize_to_bulk_string(),
        };

        Ok(response)
    }
}
