use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandGetDel {
    key: String,
}

impl RedisCommandGetDel {
    pub fn new(key: String) -> RedisCommandGetDel {
        RedisCommandGetDel { key }
    }
}

impl RedisCommand for RedisCommandGetDel {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::GetDel(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        Ok(response)
    }
}
