use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::result_error::RedisError;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandDecrBy {
    key: String,
    new_value: String,
}

impl RedisCommandDecrBy {
    pub fn new(key: String, new_value: String) -> RedisCommandDecrBy {
        RedisCommandDecrBy { key, new_value }
    }
}

impl RedisCommand for RedisCommandDecrBy {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let value = self.new_value.parse::<i32>();
        let response = match value {
            Ok(value) => {
                let response = accessor.access(StorageAction::DecrBy(self.key.clone(), value))?;
                response.get_value().protocol_serialize_to_int()
            }
            Err(_) => RedisError::NotANumber.protocol_serialize_to_bulk_string(),
        };
        Ok(response)
    }
}
