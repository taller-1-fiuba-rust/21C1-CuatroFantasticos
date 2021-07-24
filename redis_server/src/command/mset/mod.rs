use crate::command::RedisCommand;
use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandMSet {
    member_keys: Vec<String>,
    member_values: Vec<String>,
}

impl RedisCommandMSet {
    pub fn new(member_keys: Vec<String>, member_values: Vec<String>) -> RedisCommandMSet {
        RedisCommandMSet {
            member_keys,
            member_values,
        }
    }
}

impl RedisCommand for RedisCommandMSet {
    fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response = accessor.access(StorageAction::MSet(
            self.member_keys.clone(),
            self.member_values.clone(),
        ))?;
        let response = response.get_value().protocol_serialize_to_simple_string();
        Ok(response)
    }
}
