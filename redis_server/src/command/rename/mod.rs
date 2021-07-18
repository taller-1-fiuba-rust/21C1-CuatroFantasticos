use crate::command::RedisCommand;
use crate::data::storage_service::operator_service::accessor::StorageAccessor;
use crate::data::storage_service::operator_service::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandRename {
    key: String,
    new_key: String,
}

impl RedisCommandRename {
    pub fn new(key: String, new_key: String) -> RedisCommandRename {
        RedisCommandRename { key, new_key }
    }
}

impl RedisCommand for RedisCommandRename {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let rename = accessor.access(StorageAction::Rename(
            self.key.clone(),
            self.new_key.clone(),
        ))?;
        let response = rename.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}
