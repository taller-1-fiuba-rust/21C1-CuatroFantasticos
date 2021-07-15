use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageRequestMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandGetSet {
    key: String,
    new_value: String,
}

impl RedisCommandGetSet {
    pub fn new(key: String, new_value: String) -> RedisCommandGetSet {
        RedisCommandGetSet { key, new_value }
    }
}

impl RedisCommand for RedisCommandGetSet {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::GetSet(
            self.key.clone(),
            self.new_value.clone(),
        ))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        Ok(response)
    }
}
