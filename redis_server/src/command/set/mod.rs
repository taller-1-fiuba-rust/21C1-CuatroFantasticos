use crate::data::storage::service::operator::accessor::StorageAccessor;
use crate::data::storage::service::operator::request_message::StorageAction;
use crate::protocol_serialization::ProtocolSerializer;

pub struct RedisCommandSet {
    key: String,
    new_value: String,
}

impl RedisCommandSet {
    pub fn new(key: String, new_value: String) -> RedisCommandSet {
        RedisCommandSet { key, new_value }
    }
    pub fn execute(&self, accessor: StorageAccessor) -> Result<String, String> {
        let response =
            accessor.access(StorageAction::Set(self.key.clone(), self.new_value.clone()))?;
        let response = response.get_value().protocol_serialize_to_bulk_string();
        Ok(response)
    }
}
