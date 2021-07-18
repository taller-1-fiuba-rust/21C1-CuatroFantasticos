use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::data::storage_service::operator_service::response_message::StorageResult;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandType {
    key: String,
}

impl RedisCommandType {
    pub fn new(key: String) -> RedisCommandType {
        RedisCommandType { key }
    }
}

impl RedisCommand for RedisCommandType {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Type(self.key.clone()))?;
        let response = match response.get_value() {
            StorageResult::RedisValue(value) => {
                value.get_type().protocol_serialize_to_simple_string()
            }
            value => value.protocol_serialize_to_simple_string(),
        };
        Ok(response)
    }
}
