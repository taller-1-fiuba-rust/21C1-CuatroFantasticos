use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageRequestMessageEnum;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandExists {
    key: String,
}

impl RedisCommandExists {
    pub fn new(key: String) -> RedisCommandExists {
        RedisCommandExists { key }
    }
}

impl RedisCommand for RedisCommandExists {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageRequestMessageEnum::Exists(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
