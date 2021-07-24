use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSismember {
    key: String,
    member: String,
}

impl RedisCommandSismember {
    pub fn new(key: String, member: String) -> RedisCommandSismember {
        RedisCommandSismember { key, member }
    }
}

impl RedisCommand for RedisCommandSismember {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Sismember(
            self.key.clone(),
            self.member.clone(),
        ))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
