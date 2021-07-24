use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandTouch {
    key: String,
}

impl RedisCommandTouch {
    pub fn new(key: String) -> RedisCommandTouch {
        RedisCommandTouch { key }
    }
}

impl RedisCommand for RedisCommandTouch {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::Touch(self.key.clone()))?;
        let response = response.get_value().protocol_serialize_to_int();
        Ok(response)
    }
}
